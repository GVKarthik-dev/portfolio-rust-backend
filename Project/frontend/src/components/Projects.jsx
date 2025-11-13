import React, { useState, useEffect } from 'react'
import axios from 'axios'

const Projects = () => {
  const [projects, setProjects] = useState([
    {
      id: 1,
      title: 'Portfolio API',
      description: 'RESTful API built with Actix Web framework supporting CRUD operations, JWT authentication, and JSON error handling.',
      technologies: ['Rust', 'Actix Web', 'JWT', 'Bcrypt'],
      repository: 'https://github.com/example/portfolio-api',
      image: 'https://via.placeholder.com/300x200?text=Portfolio+API'
    },
    {
      id: 2,
      title: 'Portfolio Website',
      description: 'Responsive portfolio website built with React.js using modern hooks, ES6+ syntax, and component-based architecture.',
      technologies: ['React', 'ES6+', 'CSS3', 'Vite'],
      repository: 'https://github.com/example/portfolio-website',
      image: 'https://via.placeholder.com/300x200?text=Portfolio+Website'
    },
    {
      id: 3,
      title: 'E-commerce Platform',
      description: 'Full-stack e-commerce application with product management, shopping cart, and payment integration.',
      technologies: ['Rust', 'React', 'PostgreSQL', 'Stripe'],
      repository: 'https://github.com/example/ecommerce',
      image: 'https://via.placeholder.com/300x200?text=E-commerce'
    }
  ])

  useEffect(() => {
    // Fetch projects from API
    // axios.get('/api/v1/projects')
    //   .then(res => setProjects(res.data))
    //   .catch(err => console.error('Error fetching projects:', err))
  }, [])

  return (
    <section id="projects" className="projects">
      <div className="container">
        <h2 className="section-title">Featured Projects</h2>
        <div className="projects-grid">
          {projects.map((project) => (
            <div key={project.id} className="project-card">
              <div className="project-image">
                <img
                  src={project.image}
                  alt={project.title}
                  onError={(e) => { e.currentTarget.src = '/images/placeholder.svg' }}
                />
              </div>
              <div className="project-content">
                <h3 className="project-title">{project.title}</h3>
                <p className="project-description">{project.description}</p>
                <div className="project-tech">
                  {project.technologies.map((tech, idx) => (
                    <span key={idx} className="tech-badge">{tech}</span>
                  ))}
                </div>
                <div className="project-links">
                  <a href={project.repository} target="_blank" rel="noopener noreferrer" className="btn btn-small">
                    View Code
                  </a>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}

export default Projects
