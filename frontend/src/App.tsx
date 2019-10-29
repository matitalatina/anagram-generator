import React from 'react';
import styled, { createGlobalStyle } from 'styled-components';
import AnagramPage from './components/pages/AnagramPage';
import theme from './themes/default';

const GlobalStyle = createGlobalStyle`
  body {
    margin: 0;
    font-family: ${theme.fonts.primary};
    min-height: 100%;
  }
`

const App: React.SFC = () => {
  return (
    <React.Fragment>
      <GlobalStyle />
      <AnagramPage />
    </React.Fragment>
  );
}

export default App;