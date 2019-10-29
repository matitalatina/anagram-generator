import Div from '../atoms/Div';
import styled from 'styled-components';
import React, { useState, useEffect } from 'react';
import PageTemplate from '../templates/PageTemplate';
import Small from '../atoms/Small';
import ParticlesBackground from '../atoms/ParticlesBackground';
import Link from '../atoms/Link';
import Input from '../atoms/Input';
import { get_anagram } from 'anagram';
import Button from '../atoms/Button';
import Centered from '../atoms/Centered';

const Wrapper = styled<React.FunctionComponent<React.HTMLAttributes<HTMLDivElement>>>(Div)`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  flex: 1;
`

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
      <Input type="text" value={input} onChange={(event) => setInput(event.target.value)} placeholder="Frase da anagrammare..."></Input>
      {dictionary && <Button type="button" onClick={() => onNewAnagram(setAnagrams)(input, dictionary)}>Trova {anagrams.original ? 'altri' : 'gli'} anagrammi</Button>}
      {anagrams.original && <>Gli anagrammi di {anagrams.original} sono: <br />
        {anagrams.anagrams.map((w, i) => <Div key={i}>{w.join(' · ')}</Div>)}
      </>
      }
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