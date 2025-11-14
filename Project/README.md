# Portfolio Web Application

A full-stack portfolio application featuring a Rust Actix Web backend API and a React.js frontend website.

## Project Structure

```
.
├── backend/                  # Rust Actix Web REST API
│   ├── src/
│   │   ├── main.rs          # Application entry point
│   │   ├── models.rs        # Data models and request/response types
│   │   ├── errors.rs        # Error handling and JSON responses
│   │   ├── auth.rs          # JWT and password utilities
│   │   ├── db.rs            # In-memory database
│   │   └── handlers/        # HTTP request handlers
│   ├── Cargo.toml           # Rust dependencies
│   ├── .env                 # Environment variables
│   └── README.md            # Backend documentation
│
├── frontend/                # React.js Portfolio Website
│   ├── src/
│   │   ├── components/      # React components
│   │   ├── services/        # API client
│   │   ├── styles/          # CSS styling
│   │   ├── App.jsx          # Main application component
│   │   └── main.jsx         # Entry point
│   ├── index.html           # HTML template
│   ├── vite.config.js       # Vite configuration
│   ├── package.json         # Node dependencies
│   └── README.md            # Frontend documentation
│
└── README.md               # This file
```

## Quick Start

### Backend (Rust)

```bash
cd backend

# Install Rust (if not already installed)
# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

cargo build
cargo run
```

Server runs at: `http://127.0.0.1:8080`

### Frontend (React)

```bash
cd frontend

# Install Node.js dependencies
npm install

# Start development server
npm run dev
```

Website runs at: `http://localhost:3000`

## Backend Features

✅ **REST API** with CRUD operations
- User authentication (register, login)
- Project management (create, read, update, delete)
- Skills management (create, read, update, delete)

✅ **Authentication**
- User registration with password hashing
- JWT token-based authentication
- Secure password verification with bcrypt

✅ **Error Handling**
- Comprehensive JSON error responses
- Proper HTTP status codes
- Structured error messages

✅ **Architecture**
- Modular handler design
- Separated concerns (models, handlers, auth, db)
- Request/response types with serde
- In-memory database for demo purposes

## Frontend Features

✅ **Modern React Components**
- Functional components with hooks
- State management with useState
- Side effects with useEffect
- Custom component architecture

✅ **Responsive Design**
- Mobile-first CSS approach
- CSS Grid and Flexbox layouts
- Media queries for all screen sizes

✅ **Sections**
- Navigation with theme toggle
- Hero section with CTAs
- About section with stats
- Projects showcase
- Skills with proficiency bars
- Contact form
- Footer with social links

✅ **Styling**
- CSS custom properties for theming
- Light/dark mode support
- Smooth animations and transitions
- Professional UI components

## API Endpoints

### Authentication
- `POST /api/v1/auth/register` - Register new user
- `POST /api/v1/auth/login` - Login user

### Projects
- `GET /api/v1/projects` - List all projects
- `GET /api/v1/projects/{id}` - Get project by ID
- `POST /api/v1/projects` - Create project
- `PUT /api/v1/projects/{id}` - Update project
- `DELETE /api/v1/projects/{id}` - Delete project

### Skills
- `GET /api/v1/skills` - List all skills
- `GET /api/v1/skills/{id}` - Get skill by ID
- `POST /api/v1/skills` - Create skill
- `PUT /api/v1/skills/{id}` - Update skill
- `DELETE /api/v1/skills/{id}` - Delete skill

## Technologies Used

### Backend
- **Rust** - Systems programming language
- **Actix Web** - High-performance web framework
- **Serde** - Serialization/deserialization
- **jsonwebtoken** - JWT authentication
- **bcrypt** - Password hashing
- **UUID** - Unique identifiers
- **Chrono** - Date/time handling
- **Tokio** - Async runtime

### Frontend
- **React 18** - UI library
- **React Router DOM** - Client-side routing
- **Vite** - Fast build tool
- **Axios** - HTTP client
- **CSS3** - Styling and animations
- **ES6+** - Modern JavaScript

## Environment Setup

### Backend `.env` file
```
JWT_SECRET=your-secret-key-change-in-production
DATABASE_URL=sqlite::memory:
RUST_LOG=info
```

### Frontend `.env.local` file (optional)
```
VITE_API_URL=http://localhost:8080/api/v1
```

## Development Notes

### Backend Development
- All handlers return structured responses
- Errors are properly serialized to JSON
- Authentication tokens are JWT-based
- Database uses in-memory storage (can be replaced with PostgreSQL, MySQL, etc.)

### Frontend Development
- All components use modern React hooks
- API calls are centralized in `services/api.js`
- Responsive design works on all devices
- Dark mode supported globally

## Building for Production

### Backend
```bash
cd backend
cargo build --release
# Binary at: target/release/portfolio-api
```

### Frontend
```bash
cd frontend
npm run build
# Built files at: dist/
```

## Next Steps

1. **Database Integration**
   - Replace in-memory database with PostgreSQL/MongoDB
   - Implement proper schema migrations

2. **Authentication Enhancement**
   - Add refresh token rotation
   - Implement role-based access control
   - Add email verification

3. **Deployment**
   - Deploy backend to cloud (Heroku, AWS, DigitalOcean)
   - Deploy frontend to CDN (Vercel, Netlify, AWS S3)
   - Set up CI/CD pipelines

4. **Testing**
   - Add unit tests for handlers
   - Add integration tests for API
   - Add component tests for React

5. **Additional Features**
   - Blog section
   - GitHub profile integration
   - Analytics dashboard
   - Admin panel

## License

MIT

## Support

For questions or issues, please refer to the individual README files in `backend/` and `frontend/` directories.
