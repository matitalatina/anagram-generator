import { get_anagram } from 'anagram';
import React, { useEffect, useState } from 'react';
import styled from 'styled-components';
import Button from '../atoms/Button';
import Div from '../atoms/Div';
import Input from '../atoms/Input';
import Link from '../atoms/Link';
import ParticlesBackground from '../atoms/ParticlesBackground';
import Small from '../atoms/Small';
import PageTemplate from '../templates/PageTemplate';

const SpacedInput = styled(Input)`
  margin: 32px 0;
`;

const SpacedDiv = styled(Div)`
  margin: 8px 0;
`;

const SpacedButton = styled(Button)`
  margin: 0 0 16px 0;
`;

interface IAnagramFound {
  original: string,
  anagrams: string[][],
}

const AnagramPage: React.FunctionComponent = () => {
  const [anagrams, setAnagrams] = useState<IAnagramFound>({ original: '', anagrams: [] });
  const [input, setInput] = useState<string>('');
  const [dictionary, setDictionary] = useState<string[]>([]);
  useEffect(() => {
    fetch('assets/110000_parole_italiane_con_nomi_propri.txt')
      .then(res => res.text())
      .then(txt => setDictionary(txt.split('\n').filter(x => x.length > 4)));
  }, [])
  return (
    <PageTemplate
      header={null}
      footer={
        <Small transparent centered>
          Creato con <span role="img" aria-label="Heart" aria-labelledby="">❤️</span> da <Link href="https://www.mattianatali.it" target="_blank">Mattia Natali</Link> · <Link href="https://github.com/matitalatina/anagram-generator" target="_blank">Codice sorgente</Link>
        </Small>
      }
      contentCentered
    >
      <form onSubmit={(event) => {event.preventDefault(); onNewAnagram(setAnagrams)(input, dictionary)}}>
      <SpacedInput type="text" value={input} onChange={(event) => setInput(event.target.value)} placeholder="Frase da anagrammare..."></SpacedInput>
      </form>
      {dictionary && <SpacedButton type="button" onClick={() => onNewAnagram(setAnagrams)(input, dictionary)}>Trova {anagrams.original ? 'altri' : 'gli'} anagrammi</SpacedButton>}
        {anagrams.anagrams.map((w, i) => <SpacedDiv key={i}>{w.join(' · ')}</SpacedDiv>)}
        <ParticlesBackground />
    </PageTemplate>
  );
}

function onNewAnagram(setAnagrams: React.Dispatch<React.SetStateAction<IAnagramFound>>) {
  return (phrase: string, dictionary: string[]) => {
    setAnagrams({
      original: phrase,
      anagrams: getAnagrams(phrase, dictionary),
    });
  };
}

function getAnagrams(phrase: string, dictionary: string[]): string[][] {
  return get_anagram(phrase, dictionary);
}
export default AnagramPage;