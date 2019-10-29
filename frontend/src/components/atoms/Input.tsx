import styled from "styled-components";
import theme from "../../themes/default";

const Input = styled.input`
  font-family: ${theme.fonts.primary};
  font-size: ${theme.fonts.sizes.big};
  background-color: rgba(0, 0, 0, 0.1);
  border: solid 1px rgba(255, 255, 255, 0.5);
  border-radius: 15px;
  color: white;
  padding: 16px;
  outline: none;
  transition: all 0.3s;
  min-width: 500px;
  text-align: center;

  @media screen and (max-width: 600px) {
    min-width: 250px;
    font-size: ${theme.fonts.sizes.normal};
  }

  &:hover, &:focus {
    background-color: rgba(255, 255, 255, 0.05);
    border: solid 1px rgba(255, 255, 255, 0.8);
  }
`;

export default Input;