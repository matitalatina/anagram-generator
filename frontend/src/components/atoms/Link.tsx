import React from 'react'
import PropTypes from 'prop-types'
import styled, { css } from 'styled-components'
import theme from '../../themes/default'
import { NavLink } from 'react-router-dom'

const styles = css`
  font-family: ${theme.fonts.primary};
  text-decoration: none;
  font-weight: 500;
  color: ${theme.palette.grayscale[1]};

  &:hover {
    text-decoration: underline;
  }
`

const StyledNavLink = styled(({
  theme, reverse, palette, ...props
}) => <NavLink {...props} />)`${styles}`

const Anchor = styled.a`${styles}`

const Link = ({ ...props }) => {
  if (props.to) {
    return <StyledNavLink {...props} />
  }
  return <Anchor {...props} />
}

Link.propTypes = {
  palette: PropTypes.string,
  reverse: PropTypes.bool,
  to: PropTypes.string,
}

Link.defaultProps = {
  palette: 'primary',
}

export default Link
