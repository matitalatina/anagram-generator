import React from 'react'
import styled from 'styled-components'
import theme from '../../themes/default'

const Wrapper = styled.div`
  display: flex;
  flex: 1;
  align-items: center;
  justify-content: center;
  color: ${theme.palette.foreground};
`

const Centered: React.FunctionComponent<React.HTMLAttributes<HTMLDivElement>> = ({ children }) => {
  return (<Wrapper>{children}</Wrapper>)
}

export default Centered
