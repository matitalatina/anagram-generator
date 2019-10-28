import Particles from 'react-particles-js'
import styled from 'styled-components'
import React from 'react'

const StyledParticles = styled(Particles)`
  pointer-events: none;
  position: fixed;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
`

const ParticlesBackground = () => (
  <StyledParticles
    params={{
      particles: {
        number: {
          value: 40,
          density: {
            enable: true,
            value_area: 800,
          },
        },
        color: {
          value: '#ffffff',
        },
        shape: {
          type: 'circle' as const,
          stroke: {
            width: 0,
            color: '#000000',
          },
          polygon: {
            nb_sides: 5,
          },
          image: {
            src: 'img/github.svg',
            width: 100,
            height: 100,
          },
        },
        opacity: {
          value: 0.4,
          random: false,
          anim: {
            enable: true,
            speed: 1,
            opacity_min: 0.2,
            sync: false,
          },
        },
        size: {
          value: 3,
          random: true,
          anim: {
            enable: true,
            speed: 4,
            size_min: 0.1,
            sync: false,
          },
        },
        line_linked: {
          enable: true,
          distance: 70,
          color: '#ffffff',
          opacity: 0.1,
          width: 1,
        },
        move: {
          enable: true,
          speed: 2,
          direction: 'none' as const,
          random: false,
          straight: false,
          out_mode: 'out' as const,
          bounce: false,
          attract: {
            enable: false,
            rotateX: 600,
            rotateY: 1200,
          },
        },
      },
      interactivity: {
        detect_on: 'canvas',
        events: {
          onhover: {
            enable: false,
            mode: 'grab' as const,
          },
          onclick: {
            enable: false,
            mode: 'push' as const,
          },
          resize: true,
        },
        modes: {
          grab: {
            distance: 400,
            line_linked: {
              opacity: 0.2,
            },
          },
          bubble: {
            distance: 400,
            size: 0,
            duration: 2,
            opacity: 8,
          },
          repulse: {
            distance: 200,
            duration: 0.4,
          },
          push: {
            particles_nb: 20,
          },
          remove: {
            particles_nb: 2,
          },
        },
      },
      retina_detect: true,
    }}
  />
)

export default ParticlesBackground
