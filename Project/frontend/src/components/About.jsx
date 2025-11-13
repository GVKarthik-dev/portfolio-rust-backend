import React from 'react'

const About = () => {
  return (
    <section id="about" className="about">
      <div className="container">
        <h2 className="section-title">About Me</h2>
        <div className="about-content">
          <div className="about-text">
            <p>
              I'm a passionate full-stack developer with expertise in building scalable web applications.
              I specialize in Rust backend development with Actix Web framework and modern React frontends.
            </p>
            <p>
              With a focus on clean code, performance, and user experience, I create robust solutions
              that solve real-world problems.
            </p>
            <div className="about-highlights">
              <div className="highlight">
                <span className="highlight-label">Experience</span>
                <span className="highlight-value">2+ Years</span>
              </div>
              <div className="highlight">
                <span className="highlight-label">Projects</span>
                <span className="highlight-value">20+</span>
              </div>
              <div className="highlight">
                <span className="highlight-label">Technologies</span>
                <span className="highlight-value">15+</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}

export default About
