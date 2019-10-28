import styled from 'styled-components'
import theme from '../../themes/default'

type SmallProps = {
  transparent?: boolean,
  centered?: boolean,
}

const Small: React.FunctionComponent<SmallProps> = styled.small<SmallProps>`
  font-family: ${theme.fonts.primary};
  color: ${theme.palette.foreground};
  opacity: ${props => props.transparent ? 0.4 : 1};
  text-align: ${props => props.centered ? 'center' : 'auto'};
  font-size: 0.6rem;

  &:hover {
    opacity: 1;
  }
`

export default Small
