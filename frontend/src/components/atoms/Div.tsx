import React from "react";
import PropTypes from 'prop-types'
import styled from 'styled-components'
import theme from '../../themes/default'

const Div: React.FunctionComponent = styled.div`
  font-family: ${theme.fonts.primary};
  color: ${theme.palette.foreground};
`

export default Div
