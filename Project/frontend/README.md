# Portfolio Website - React.js Frontend

A responsive portfolio website built with React.js featuring modern hooks, ES6+ syntax, and beautiful UI components.

## Features

- **Modern React Hooks**: Uses functional components with useState, useEffect, and custom hooks
- **ES6+ Syntax**: Arrow functions, destructuring, template literals, and more
- **Responsive Design**: Mobile-first approach with CSS Grid and Flexbox
- **Dark Mode**: Light/dark theme toggle
- **Section Components**:
  - Hero section with call-to-action buttons
  - About section with highlights
  - Projects showcase with filtering
  - Skills visualization with proficiency levels
  - Contact form with validation
  - Footer with social links

## Technologies

- **React 18** - UI library
- **React Router DOM** - Client-side routing
- **Vite** - Build tool and dev server
- **Axios** - HTTP client
- **CSS3** - Styling and animations

## Getting Started

### Prerequisites
- Node.js 16.x or higher
- npm or yarn

### Installation

1. Navigate to the frontend directory:
```bash
cd frontend
```

2. Install dependencies:
```bash
npm install
```

3. Start the development server:
```bash
npm run dev
```

The website will open at `http://localhost:3000`

## Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run preview` - Preview production build locally

## Project Structure

```
frontend/
├── src/
│   ├── components/
│   │   ├── Navigation.jsx
│   │   ├── Hero.jsx
│   │   ├── About.jsx
│   │   ├── Projects.jsx
│   │   ├── Skills.jsx
│   │   ├── Contact.jsx
│   │   └── Footer.jsx
│   ├── services/
│   │   └── api.js
│   ├── styles/
│   │   └── index.css
│   ├── App.jsx
│   └── main.jsx
├── index.html
├── vite.config.js
└── package.json
```

## Features Breakdown

### Components

- **Navigation**: Sticky header with theme toggle
- **Hero**: Eye-catching introduction section
- **About**: Bio with key statistics
- **Projects**: Grid layout showcasing portfolio projects
- **Skills**: Categorized skills with proficiency bars
- **Contact**: Contact form and social links
- **Footer**: Quick navigation and credits

### Styling

- CSS custom properties (variables) for theming
- Dark mode support
- Smooth animations and transitions
- Mobile-responsive design
- Hover effects and interactive elements

## API Integration

The frontend is configured to connect to the backend API at `http://localhost:8080/api/v1`

Example API calls:
```javascript
// In components, you can use the API services:
import { projectsAPI, skillsAPI } from './services/api'

// Fetch projects
const projects = await projectsAPI.list()

// Fetch skills
const skills = await skillsAPI.list()
```

## Environment Variables

Create a `.env.local` file in the frontend directory:
```
VITE_API_URL=http://localhost:8080/api/v1
```

## Building for Production

```bash
npm run build
```

This will create an optimized production build in the `dist/` directory.

## License

MIT
