import React from 'react'
import styled from 'styled-components'
import theme from '../../themes/default'

const Wrapper = styled.div`
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  box-sizing: border-box;
  background-color: ${theme.palette.background};
  color: ${theme.palette.foreground};
`

const Header = styled.header`
  position: fixed;
  top: 0;
  width: 100%;
  z-index: 999;
`

const Hero = styled.section``

const Sponsor = styled.section``

const Content = styled.section`
  width: 100%;
  box-sizing: border-box;
  margin: 2rem auto;
`

const ContentCentered = styled(Content)`
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
`

const Footer = styled.footer`
  padding: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: row;
`

type PageTemplateProps = {
  header?: React.ReactNode,
  hero?: React.ReactNode,
  sponsor?: React.ReactNode,
  footer?: React.ReactNode,
  contentCentered?: boolean,
}

const PageTemplate: React.FunctionComponent<PageTemplateProps> = ({
  header, hero, sponsor, children, footer, contentCentered = false, ...props
}) => {
  return (
    <Wrapper {...props}>
      {header && <Header>{header}</Header>}
      {hero && <Hero>{hero}</Hero>}
      {sponsor && <Sponsor>{sponsor}</Sponsor>}
      {contentCentered ? <ContentCentered>{children}</ContentCentered> : <Content>{children}</Content>}
      {footer && <Footer>{footer}</Footer>}
    </Wrapper>
  )
}

export default PageTemplate
