import React, { useState } from 'react'

const Navigation = ({ isDark, toggleDark }) => {
  const [isMenuOpen, setIsMenuOpen] = useState(false)

  return (
    <nav className="navbar">
      <div className="nav-container">
        <div className="nav-logo">
          <span className="logo-text">💼 Portfolio</span>
        </div>
        
        <button 
          className="menu-toggle"
          onClick={() => setIsMenuOpen(!isMenuOpen)}
          aria-label="Toggle menu"
        >
          {isMenuOpen ? '✕' : '☰'}
        </button>

        <ul className={`nav-menu ${isMenuOpen ? 'active' : ''}`}>
          <li className="nav-item">
            <a href="#about" className="nav-link" onClick={() => setIsMenuOpen(false)}>About</a>
          </li>
          <li className="nav-item">
            <a href="#projects" className="nav-link" onClick={() => setIsMenuOpen(false)}>Projects</a>
          </li>
          <li className="nav-item">
            <a href="#skills" className="nav-link" onClick={() => setIsMenuOpen(false)}>Skills</a>
          </li>
          <li className="nav-item">
            <a href="#contact" className="nav-link" onClick={() => setIsMenuOpen(false)}>Contact</a>
          </li>
          <li className="nav-item">
            <button 
              className="theme-toggle" 
              onClick={toggleDark}
              aria-label="Toggle theme"
              title={isDark ? 'Light mode' : 'Dark mode'}
            >
              {isDark ? '☀️' : '🌙'}
            </button>
          </li>
        </ul>
      </div>
    </nav>
  )
}

export default Navigation
