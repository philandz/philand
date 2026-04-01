# 🏦 Philand v1.0.0

Modern budget tracking and financial management platform built with **Rust** + **Next.js**.

> Migration note: this repository is the legacy/monolith application (v1) used during strangler-pattern extraction.
> New microservices are developed in sibling repositories (`identity`, `gateway`, `protobuf`, `libs`, `infra`).

<div align="center">

![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)
![Next.js](https://img.shields.io/badge/next.js-14.2+-black.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

</div>

## ✨ Features

- 🔐 **Secure Authentication** - JWT + bcrypt with role-based access
- 💰 **Multi-Budget Management** - Personal and team budgets with categories
- 👥 **Team Collaboration** - Invite members with granular permissions
- 📊 **Analytics & Reports** - Visual charts and monthly summaries
- 🌐 **Modern UI/UX** - Responsive design with dark/light themes
- 🌍 **Internationalization** - English and Vietnamese support
- 📱 **Mobile Optimized** - PWA with intuitive mobile navigation

## 🏗️ Tech Stack

**Backend (Rust)**: Axum + SQLx + MySQL + JWT  
**Frontend (Next.js)**: React 18 + TypeScript + Tailwind + Radix UI  
**Infrastructure**: Docker + Nginx + MySQL

## 🚀 Quick Start

### Docker Setup (Recommended)
```bash
git clone https://github.com/fissama/philand.git
cd philand

# Copy environment files
cp .env.example .env
cp web/.env.example web/.env.local

# Start all services
docker-compose up -d --build

# Access the app at http://localhost:3000
```

### Local Development
```bash
# 1. Start database
docker-compose -f docker-compose.dev.yml up -d database

# 2. Run migrations
cargo sqlx migrate run

# 3. Start backend
cargo run

# 4. Start frontend
cd web && npm install && npm run dev
```

**Access Points:**
- Frontend: http://localhost:3000
- Backend API: http://localhost:8080
- Database: localhost:3306 (philand/philand)

## ⚙️ Configuration

Key environment variables (see `.env.example` for full list):

```bash
# Database
DB_URL="mysql://user:pass@host:port/philand"

# Security
JWT_SECRET="your-secret-key"
BCRYPT_COST=12

# API
NET_PORT=8080
CORS_ORIGINS=http://localhost:3000

# Frontend
NEXT_PUBLIC_API_URL=http://localhost:8080
```

## 🔐 Security & Roles

| Role | Permissions |
|------|-------------|
| **Owner** | Full control, manage members, delete budget |
| **Manager** | Manage categories and settings, view all data |
| **Contributor** | Add/edit transactions, view budget data |
| **Viewer** | Read-only access to budget and summaries |

## 📡 API Endpoints

**Authentication**: `/auth/signup`, `/auth/login`, `/auth/reset`  
**Budgets**: `/api/budgets` - CRUD operations  
**Transactions**: `/api/budgets/:id/entries` - Transaction management  
**Categories**: `/api/budgets/:id/categories` - Category management  
**Members**: `/api/budgets/:id/members` - Team management  
**Analytics**: `/api/budgets/:id/summary/monthly` - Reports and summaries

> See [API Documentation](docs/api.md) for detailed endpoint specifications.

## 🚀 Production Deployment

```bash
# Quick Docker deployment
./scripts/prod.sh

# Manual deployment
docker-compose up -d --build

# Traditional deployment
cargo build --release && ./target/release/philand
cd web && npm run build && npm start
```

**Production Requirements:**
- MySQL 8.0+ with backups
- SSL certificates (Let's Encrypt)
- Reverse proxy (Nginx recommended)
- Environment variables configured

## 🛠️ Development

```bash
# Backend development
cargo watch -x run
cargo test
cargo clippy

# Frontend development  
cd web && npm run dev
npm run type-check
npm run lint

# Docker development
./scripts/dev.sh
docker-compose -f docker-compose.dev.yml up -d
```

## 🗺️ Roadmap

Philand follows a structured development approach with clearly defined phases, each building upon the previous to create a comprehensive financial management platform.

### 🎯 Phase 1: Enhanced Budget Management (Q4 2025)
**Budget Types & Entry Enhancements**
- [ ] **Budget Types**: Personal, Shared, Business, Project budgets with UI badges
- [ ] **Entry Comments**: Add detailed notes and context to transactions
- [ ] **Entry Tags**: Flexible tagging system for better organization
- [ ] **Advanced Filtering**: Filter by tags, comments, and budget types
- [ ] **Bulk Operations**: Mass edit tags and comments on multiple entries

### 🤝 Phase 2: Collaborative Finance (Q1 2026)
**Sharing & Split Management**
- [ ] **Budget Sharing**: Share budgets with external users (payer system)
- [ ] **Equal Split**: Automatic expense splitting among participants
- [ ] **Balance Tracking**: Real-time balance calculations between members
- [ ] **Settlement System**: Track who owes whom and settlement history
- [ ] **Split Notifications**: Alerts for new shared expenses and settlements

### 💰 Phase 3: Specialized Financial Profiles (Q2 2026)
**Advanced Financial Management**
- [ ] **Savings Tracking**: Dedicated savings goals and progress monitoring
- [ ] **Debt Management**: Debt tracking with payment schedules and interest calculations
- [ ] **Investment Portfolio**: Basic investment tracking and performance metrics
- [ ] **Financial Profiles**: Customizable profiles for different financial behaviors
- [ ] **Goal Setting**: SMART financial goals with milestone tracking

### 🔄 Phase 4: Advanced Operations (Q3 2026)
**Transaction Safety & Transfers**
- [ ] **Inter-Budget Transfers**: Safe transfers between budgets with transaction integrity
- [ ] **Transfer History**: Complete audit trail of all transfers
- [ ] **Batch Transfers**: Multiple transfers in a single transaction
- [ ] **Transfer Approval**: Multi-step approval process for large transfers
- [ ] **Rollback System**: Safe rollback of transfers with proper validation

### 📋 Phase 5: Audit & Compliance (Q4 2026)
**Complete Audit System**
- [ ] **Comprehensive Audit Log**: Track all important actions and changes
- [ ] **User Activity Tracking**: Detailed logs of user interactions
- [ ] **Data Export**: Export audit logs for compliance and analysis
- [ ] **Retention Policies**: Configurable data retention and archival
- [ ] **Compliance Reports**: Generate reports for financial auditing

### 🚀 Future Enhancements (2027+)
**Advanced Features**
- [ ] **Mobile Applications**: Native iOS and Android apps
- [ ] **API Integrations**: Bank account synchronization and fintech partnerships
- [ ] **Advanced Analytics**: AI-powered insights and spending predictions
- [ ] **Multi-Currency**: Advanced currency conversion and international support
- [ ] **Enterprise Features**: SSO, advanced security, and enterprise-grade features

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

**Areas for contribution:**
- 🐛 Bug fixes and improvements
- ✨ New features from roadmap
- 📚 Documentation and guides
- 🌍 Translations and i18n
- 🎨 UI/UX enhancements

## 📄 License

**MIT License** © 2025 Philand Project

---

<div align="center">

**Built with ❤️ by the Philand Team**

[Website](https://www.phila.cloud) • [Documentation](https://www.phila.cloud/docs) • [GitHub](https://github.com/fissama/philand)

</div>
