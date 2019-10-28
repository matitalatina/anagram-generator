import React from 'react';
import { createGlobalStyle } from 'styled-components';
import AnagramPage from './components/pages/AnagramPage';
import theme from './themes/default';

const GlobalStyle = createGlobalStyle`
  body {
    margin: 0;
    font-family: ${theme.fonts.primary}
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