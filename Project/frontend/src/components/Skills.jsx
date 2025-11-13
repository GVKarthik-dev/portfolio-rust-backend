import React, { useState, useEffect } from 'react'
import axios from 'axios'

const Skills = () => {
  const [skills, setSkills] = useState({
    backend: [
      { name: 'Rust', proficiency: 'advanced' },
      { name: 'Actix Web', proficiency: 'advanced' },
      { name: 'RESTful APIs', proficiency: 'expert' },
      { name: 'JWT Authentication', proficiency: 'advanced' },
      { name: 'PostgreSQL', proficiency: 'advanced' },
      { name: 'MongoDB', proficiency: 'intermediate' }
    ],
    frontend: [
      { name: 'React.js', proficiency: 'expert' },
      { name: 'JavaScript/ES6+', proficiency: 'expert' },
      { name: 'React Hooks', proficiency: 'expert' },
      { name: 'CSS3', proficiency: 'advanced' },
      { name: 'Responsive Design', proficiency: 'expert' },
      { name: 'Vite', proficiency: 'advanced' }
    ],
    tools: [
      { name: 'Git', proficiency: 'expert' },
      { name: 'Docker', proficiency: 'intermediate' },
      { name: 'VS Code', proficiency: 'expert' },
      { name: 'npm/yarn', proficiency: 'expert' },
      { name: 'Linux', proficiency: 'advanced' },
      { name: 'Cargo', proficiency: 'advanced' }
    ]
  })

  const getProficiencyColor = (proficiency) => {
    const colors = {
      beginner: '#ff6b6b',
      intermediate: '#ffd93d',
      advanced: '#6bcf7f',
      expert: '#4ecdc4'
    }
    return colors[proficiency] || '#999'
  }

  return (
    <section id="skills" className="skills">
      <div className="container">
        <h2 className="section-title">Skills</h2>
        <div className="skills-container">
          {Object.entries(skills).map(([category, categorySkills]) => (
            <div key={category} className="skill-category">
              <h3 className="category-title">{category.charAt(0).toUpperCase() + category.slice(1)}</h3>
              <div className="skills-list">
                {categorySkills.map((skill, idx) => (
                  <div key={idx} className="skill-item">
                    <div className="skill-header">
                      <span className="skill-name">{skill.name}</span>
                      <span className="skill-level" style={{ color: getProficiencyColor(skill.proficiency) }}>
                        {skill.proficiency}
                      </span>
                    </div>
                    <div className="skill-bar">
                      <div 
                        className="skill-fill" 
                        style={{
                          width: skill.proficiency === 'beginner' ? '25%' : 
                                 skill.proficiency === 'intermediate' ? '50%' :
                                 skill.proficiency === 'advanced' ? '75%' : '100%',
                          backgroundColor: getProficiencyColor(skill.proficiency)
                        }}
                      ></div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}

export default Skills
