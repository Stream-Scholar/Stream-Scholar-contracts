#!/bin/bash

# Documentation Quality Check Script
# This script enforces enterprise-grade documentation standards
# for the Stream Scholar contracts repository

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
MIN_DOC_COVERAGE=95  # Minimum documentation coverage percentage
MAX_UNDOCUMENTED_PUBLIC=0  # Maximum allowed undocumented public items

echo -e "${GREEN}🔍 Running Documentation Quality Checks${NC}"
echo "============================================="

# Function to check if a line contains a public item
is_public_item() {
    local line="$1"
    [[ "$line" =~ ^[[:space:]]*pub[[:space:]]+(struct|enum|fn|trait|impl|mod|const|type)[[:space:]] ]]
}

# Function to check if a public item has documentation
has_documentation() {
    local line="$1"
    local prev_line="$2"
    [[ "$prev_line" =~ ^[[:space:]]*///[[:space:]] ]] || [[ "$line" =~ ^[[:space:]]*///[[:space:]] ]]
}

# Function to check doc-comment quality
check_doc_quality() {
    local file="$1"
    local line_num="$2"
    local doc_lines=("$@")
    
    local quality_issues=()
    
    # Check for required sections in function documentation
    local has_requirements=false
    local has_returns=false
    local has_errors=false
    local has_examples=false
    
    for doc_line in "${doc_lines[@]}"; do
        if [[ "$doc_line" =~ .*#[[:space:]]*Requirements.*|[[:space:]]*Input.* ]]; then
            has_requirements=true
        fi
        if [[ "$doc_line" =~ .*#[[:space:]]*Returns.*|[[:space:]]*Output.* ]]; then
            has_returns=true
        fi
        if [[ "$doc_line" =~ .*#[[:space:]]*Errors.*|[[:space:]]*Panics.* ]]; then
            has_errors=true
        fi
        if [[ "$doc_line" =~ .*#[[:space:]]*Example.*|[[:space:]]*Examples.* ]]; then
            has_examples=true
        fi
    done
    
    # Public functions should have comprehensive documentation
    if [[ "$file" =~ .*\.rs$ ]]; then
        if [[ "$has_requirements" == false ]] || [[ "$has_returns" == false ]]; then
            quality_issues+=("Missing required documentation sections")
        fi
    fi
    
    echo "${quality_issues[@]}"
}

# Main analysis
total_public_items=0
undocumented_public_items=0
quality_issues_total=0
files_with_issues=()

echo "Analyzing Rust source files..."
echo

# Find all Rust files
rust_files=$(find . -name "*.rs" -not -path "./target/*" -not -path "./.git/*")

for file in $rust_files; do
    echo -e "${YELLOW}📄 Checking: $file${NC}"
    
    file_public_items=0
    file_undocumented=0
    file_quality_issues=0
    in_doc_block=false
    doc_lines=()
    
    while IFS= read -r line; do
        line_num=$((line_num + 1))
        
        # Track doc blocks
        if [[ "$line" =~ ^[[:space:]]*///[[:space:]] ]]; then
            in_doc_block=true
            doc_lines+=("$line")
            continue
        elif [[ "$line" =~ ^[[:space:]]*[^/[:space:]] ]] && [[ "$in_doc_block" == true ]]; then
            in_doc_block=false
        fi
        
        # Check for public items
        if is_public_item "$line"; then
            file_public_items=$((file_public_items + 1))
            total_public_items=$((total_public_items + 1))
            
            # Get previous line for doc check
            prev_line=$(sed -n "$((line_num - 1))p" "$file" 2>/dev/null || echo "")
            
            if ! has_documentation "$line" "$prev_line"; then
                file_undocumented=$((file_undocumented + 1))
                undocumented_public_items=$((undocumented_public_items + 1))
                
                echo -e "  ${RED}❌ Line $line_num: Undocumented public item${NC}"
                echo -e "     $line"
            else
                # Check documentation quality
                quality_issues=($(check_doc_quality "$file" "$line_num" "${doc_lines[@]}"))
                if [[ ${#quality_issues[@]} -gt 0 ]]; then
                    file_quality_issues=$((file_quality_issues + ${#quality_issues[@]}))
                    quality_issues_total=$((quality_issues_total + ${#quality_issues[@]}))
                    
                    echo -e "  ${YELLOW}⚠️  Line $line_num: Documentation quality issues${NC}"
                    for issue in "${quality_issues[@]}"; do
                        echo -e "     - $issue"
                    done
                fi
            fi
            
            # Reset doc lines for next item
            doc_lines=()
        fi
    done < "$file"
    
    # Report file summary
    if [[ $file_undocumented -gt 0 ]] || [[ $file_quality_issues -gt 0 ]]; then
        files_with_issues+=("$file")
        echo -e "  ${RED}📊 Summary: $file_undocumented undocumented, $file_quality_issues quality issues${NC}"
    else
        echo -e "  ${GREEN}✅ All public items properly documented${NC}"
    fi
    
    echo
done

# Calculate coverage
if [[ $total_public_items -gt 0 ]]; then
    coverage=$(( (total_public_items - undocumented_public_items) * 100 / total_public_items ))
else
    coverage=100
fi

# Final report
echo "============================================="
echo -e "${GREEN}📊 Documentation Quality Report${NC}"
echo "============================================="
echo "Total public items: $total_public_items"
echo "Undocumented items: $undocumented_public_items"
echo "Documentation coverage: ${coverage}%"
echo "Quality issues found: $quality_issues_total"
echo "Files with issues: ${#files_with_issues[@]}"

# Check thresholds
exit_code=0

if [[ $undocumented_public_items -gt $MAX_UNDOCUMENTED_PUBLIC ]]; then
    echo -e "\n${RED}❌ FAIL: Too many undocumented public items ($undocumented_public_items > $MAX_UNDOCUMENTED_PUBLIC)${NC}"
    exit_code=1
fi

if [[ $coverage -lt $MIN_DOC_COVERAGE ]]; then
    echo -e "\n${RED}❌ FAIL: Documentation coverage too low (${coverage}% < ${MIN_DOC_COVERAGE}%)${NC}"
    exit_code=1
fi

if [[ $quality_issues_total -gt 0 ]]; then
    echo -e "\n${YELLOW}⚠️  WARNING: $quality_issues_total documentation quality issues found${NC}"
    echo "Consider improving documentation for enterprise-grade standards"
fi

# List files with issues
if [[ ${#files_with_issues[@]} -gt 0 ]]; then
    echo -e "\n${YELLOW}Files requiring attention:${NC}"
    for file in "${files_with_issues[@]}"; do
        echo "  - $file"
    done
fi

# Success message
if [[ $exit_code -eq 0 ]]; then
    echo -e "\n${GREEN}🎉 SUCCESS: Documentation meets enterprise-grade standards!${NC}"
    echo "✅ All public items properly documented"
    echo "✅ Documentation coverage: ${coverage}%"
    echo "✅ Ready for professional security audit"
else
    echo -e "\n${RED}🚫 FAILURE: Documentation does not meet standards${NC}"
    echo "❌ Please address the issues before proceeding with audit"
fi

echo
echo "============================================="
echo "Documentation Quality Check Complete"
echo "============================================="

exit $exit_code
