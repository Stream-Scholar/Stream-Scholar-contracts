# Stream-Scholar NFT Contracts

A Stellar-native NFT system that creates dynamic Student Profile NFTs which grow in "Level" as students learn and achieve milestones.

## 🎓 Overview

Stream-Scholar NFT transforms a student's learning journey into a unique, tradable non-fungible token on the Stellar blockchain. Each NFT represents a student's profile that dynamically evolves based on their educational achievements, course completions, and skill development.

### Key Features

- **Dynamic Leveling**: NFTs automatically level up (1-8) based on accumulated XP
- **Achievement System**: Unlock badges and achievements that add visual flair to your NFT
- **Course Tracking**: Link completed courses to earn XP and level up
- **Study Streaks**: Maintain daily study habits for bonus rewards
- **Visual Progression**: NFT artwork changes based on level and achievements
- **Stellar Native**: Built on Stellar for low fees and fast transactions
- **Transferable**: Own, trade, or gift your learning profile NFT

## 🚀 Quick Start

### Prerequisites

- Node.js 16+ 
- Stellar account (testnet for development)
- Basic understanding of blockchain concepts

### Installation

```bash
# Clone the repository
git clone https://github.com/damzempire/Stream-Scholar-contracts.git
cd Stream-Scholar-contracts

# Install dependencies
npm install
```

### Environment Setup

Create a `.env` file with your Stellar account credentials:

```env
# Testnet Deployer Account
DEPLOYER_SECRET=YOUR_DEPLOYER_SECRET_KEY

# Testnet Minter Account  
MINTER_SECRET=YOUR_MINTER_SECRET_KEY
```

> **Note**: Use testnet accounts for development. Get testnet XLM from the [Stellar Friendbot](https://friendbot.stellar.org/).

### Deploy Contract

```bash
# Deploy the NFT contract to Stellar testnet
npm run deploy
```

### Mint Your First NFT

```bash
# Mint a student profile NFT
npm run mint student123 "Alice Johnson" alice@example.com

# Or use interactive mode
npm run mint -- --interactive
```

### Run Frontend

```bash
# Start the development server
npm run dev

# Open http://localhost:8080 in your browser
```

## 📊 Level System

Students progress through 8 distinct levels based on accumulated XP:

| Level | Name | Required XP | Color | Icon |
|-------|------|-------------|-------|------|
| 1 | Beginner | 0 | Gray | 🌱 |
| 2 | Novice | 100 | Silver | 📖 |
| 3 | Apprentice | 250 | Bronze | ⚒️ |
| 4 | Scholar | 500 | Gold | 🎓 |
| 5 | Expert | 1,000 | Emerald | 💎 |
| 6 | Master | 2,000 | Blue | 👑 |
| 7 | Grandmaster | 5,000 | Purple | 🔮 |
| 8 | Legend | 10,000 | Orange | 🏆 |

### XP Earning Methods

- **Course Completion**: 50-300 XP based on difficulty
- **Achievements**: 10-500 XP per achievement
- **Study Streaks**: 2 XP per consecutive day
- **Skill Development**: 25-100 XP per new skill
- **Peer Recognition**: 5-25 XP for endorsements

## 🏆 Achievement System

Achievements are automatically unlocked based on student activities:

### Course Achievements
- `first_course` - Complete your first course (25 XP)
- `courses_5` - Complete 5 courses (50 XP)
- `courses_10` - Complete 10 courses (100 XP)
- `courses_25` - Complete 25 courses (250 XP)

### Streak Achievements  
- `streak_7` - 7 day study streak (14 XP)
- `streak_30` - 30 day study streak (60 XP)
- `streak_100` - 100 day study streak (200 XP)
- `streak_365` - 365 day study streak (730 XP)

### Skill Achievements
- `first_skill` - Acquire your first skill (25 XP)
- `skill_master` - Master a skill (100 XP)
- `polymath` - Acquire 10+ skills (200 XP)

## 🎨 NFT Visual Design

Each Student Profile NFT features dynamic SVG artwork that changes based on:

- **Level**: Background gradient and level badge color
- **Progress**: Animated progress bar showing XP to next level
- **Achievements**: Visual badges displayed on the NFT
- **Study Streak**: Fire indicator for active streaks
- **Special Status**: Unique effects for legendary students

### NFT Metadata Structure

```json
{
  "name": "Stream-Scholar Profile: student123",
  "description": "A dynamic NFT representing the learning journey...",
  "image": "data:image/svg+xml;base64,...",
  "attributes": [
    {
      "trait_type": "Level",
      "value": 4
    },
    {
      "trait_type": "Title", 
      "value": "Scholar"
    },
    {
      "trait_type": "XP",
      "value": 750
    }
  ],
  "properties": {
    "level_color": "#FFD700",
    "next_level_xp": 1000,
    "progress_to_next": 0.5
  }
}
```

## 🛠️ Architecture

### Smart Contract (`contracts/StudentProfileNFT.js`)

The core Stellar smart contract handles:
- NFT creation and minting
- XP updates and level calculations
- Achievement tracking
- Ownership transfers
- Metadata management

### Student Profile (`src/StudentProfile.js`)

Comprehensive profile management:
- Personal information storage
- Learning progress tracking
- Achievement system
- Course and skill management
- Social features

### Frontend (`frontend/`)

Modern web interface built with:
- HTML5/CSS3 with Tailwind CSS
- Vanilla JavaScript with Stellar SDK
- Responsive design
- Real-time updates
- Interactive NFT display

## 📖 API Reference

### StudentProfileNFT Class

```javascript
const nftContract = new StudentProfileNFT(network, horizonUrl);

// Deploy contract
await nftContract.deployContract(deployerKeypair);

// Mint NFT
const result = await nftContract.mintNFT(studentId, metadata, issuerKeypair);

// Update XP
const updated = await nftContract.updateXP(studentId, xpAmount, signerKeypair);

// Add achievement
await nftContract.addAchievement(studentId, achievement, signerKeypair);
```

### StudentProfile Class

```javascript
const profile = new StudentProfile(studentId, initialData);

// Add XP
const result = profile.addXP(100, 'course_completion');

// Add achievement
profile.addAchievement({
  title: 'Fast Learner',
  description: 'Complete 3 courses in one week',
  xpReward: 150
});

// Update course progress
profile.updateCourseProgress(courseId, 75, false);

// Get statistics
const stats = profile.getStats();
```

## 🔧 Configuration

### Network Settings

```javascript
// Testnet
const network = Networks.TESTNET;
const horizonUrl = 'https://horizon-testnet.stellar.org';

// Mainnet (for production)
const network = Networks.PUBLIC;
const horizonUrl = 'https://horizon.stellar.org';
```

### Contract Configuration

```javascript
const config = {
  maxLevel: 8,
  baseXP: 100,
  achievementMultiplier: 1.5,
  streakBonus: 2,
  maxCourses: 100,
  maxAchievements: 50
};
```

## 🧪 Testing

Run the test suite:

```bash
# Run all tests
npm test

# Run specific test file
npm test -- tests/StudentProfile.test.js

# Run with coverage
npm run test:coverage
```

### Test Coverage

- ✅ Student profile creation and management
- ✅ XP calculation and leveling system
- ✅ Achievement unlocking logic
- ✅ Course progress tracking
- ✅ NFT minting and metadata
- ✅ Stellar transaction handling

## 📱 Frontend Features

### Minting Interface
- Form validation and preview
- Real-time NFT visualization
- Wallet connection (Stellar testnet)
- Transaction status tracking

### Profile Management
- Dynamic NFT display with animations
- XP and level progress visualization
- Achievement showcase
- Course and skill tracking
- Activity timeline

### Interactive Features
- Add XP manually for testing
- Create custom achievements
- Transfer NFT ownership
- Export profile data

## 🔐 Security Considerations

- **Private Key Management**: Never expose secret keys in frontend code
- **Input Validation**: All user inputs are validated before blockchain transactions
- **Access Control**: Only NFT owners can update their profile
- **Rate Limiting**: Implement rate limiting for XP updates to prevent abuse

## 🌐 Integration Guide

### Connecting Your Learning Platform

```javascript
// Initialize Stream-Scholar integration
const streamScholar = new StreamScholarNFT();

// Connect student wallet
await streamScholar.connectWallet();

// Award XP for course completion
await streamScholar.addXP(courseXP, 'course_completion', {
  courseId: 'course123',
  title: 'Introduction to Blockchain'
});

// Unlock achievement
await streamScholar.addAchievement({
  id: 'blockchain_basics',
  title: 'Blockchain Basics',
  description: 'Complete blockchain fundamentals course',
  icon: '⛓️',
  xpReward: 100
});
```

### Webhook Integration

Set up webhooks to receive real-time updates:

```javascript
// Webhook endpoint for XP updates
app.post('/webhook/xp-updated', (req, res) => {
  const { studentId, newXp, newLevel } = req.body;
  
  // Update your platform's records
  updateStudentProgress(studentId, { xp: newXp, level: newLevel });
  
  res.json({ success: true });
});
```

## 📊 Analytics & Monitoring

### Tracking Student Progress

```javascript
// Get student statistics
const stats = profile.getStats();
console.log(`
  Total XP: ${stats.totalXP}
  Current Level: ${stats.level}
  Courses Completed: ${stats.coursesCompleted}
  Achievements: ${stats.achievementsUnlocked}
  Study Streak: ${stats.studyStreak} days
`);
```

### Platform Analytics

```javascript
// Get platform-wide analytics
const analytics = await getPlatformAnalytics();
console.log(`
  Total Students: ${analytics.totalStudents}
  Active NFTs: ${analytics.activeNFTs}
  Average Level: ${analytics.averageLevel}
  Total XP Awarded: ${analytics.totalXPAwarded}
`);
```

## 🚀 Deployment

### Production Deployment

1. **Environment Setup**
   ```bash
   export NETWORK=public
   export HORIZON_URL=https://horizon.stellar.org
   ```

2. **Contract Deployment**
   ```bash
   npm run deploy:prod
   ```

3. **Frontend Deployment**
   ```bash
   npm run build
   # Deploy dist/ folder to your hosting provider
   ```

### Monitoring

Set up monitoring for:
- Transaction success rates
- Gas fee optimization
- User engagement metrics
- Error tracking and alerts

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Documentation**: [docs.streamscholar.io](https://docs.streamscholar.io)
- **Discord Community**: [Join our Discord](https://discord.gg/streamscholar)
- **Issues**: [GitHub Issues](https://github.com/damzempire/Stream-Scholar-contracts/issues)
- **Email**: support@streamscholar.io

## 🗺️ Roadmap

### Phase 1 - Core Features ✅
- [x] Basic NFT minting
- [x] Level progression system
- [x] Achievement tracking
- [x] Frontend interface

### Phase 2 - Enhanced Features (In Progress)
- [ ] Social features (following, leaderboards)
- [ ] Course marketplace integration
- [ ] Advanced analytics dashboard
- [ ] Mobile app development

### Phase 3 - Ecosystem (Future)
- [ ] DAO governance for achievement standards
- [ ] Cross-chain compatibility
- [ ] Institutional partnerships
- [ ] Scholarship programs

## 📈 Success Metrics

- **Student Engagement**: Daily active users and study streaks
- **Learning Outcomes**: Course completion rates and skill acquisition
- **NFT Adoption**: Number of minted profiles and trading volume
- **Platform Growth**: New student registrations and retention rates

---

**Built with ❤️ by the Stream-Scholar Team**

*Transforming education into verifiable digital achievements on the Stellar blockchain.*
