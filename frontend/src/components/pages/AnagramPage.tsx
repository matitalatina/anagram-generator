import Div from '../atoms/Div';
import styled from 'styled-components';
import React, { useState, useEffect } from 'react';
import PageTemplate from '../templates/PageTemplate';
import Small from '../atoms/Small';
import ParticlesBackground from '../atoms/ParticlesBackground';
import Link from '../atoms/Link';
import {get_anagram} from 'anagram';

const Wrapper = styled<React.FunctionComponent<React.HTMLAttributes<HTMLDivElement>>>(Div)`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  flex: 1;
`

const AnagramPage: React.FunctionComponent = () => {
  const [anagrams, setAnagrams] = useState<string[][]>([]);
  useEffect(() => {
    fetch('assets/110000_parole_italiane_con_nomi_propri.txt')
    .then(res => res.text())
    .then(txt => {
      setAnagrams(get_anagram('Mattia Natali', txt.split('\n').filter(x => x.length > 4)));
    });
  }, [])
  return (
    <PageTemplate
        header={null}
        footer={
          <Small transparent centered>
            Creato con <span role="img" aria-label="Heart" aria-labelledby="">❤️</span> da <Link href="https://www.mattianatali.it" target="_blank">Mattia Natali</Link> · <Link href="https://www.mattianatali.it" target="_blank">Codice sorgente</Link>
          </Small>
        }
        contentCentered
      >
        Gli anagrammi di Mattia Natali sono: <br />
        {anagrams.map((w, i) => <Div key={i}>{w.join(',')}</Div>)}
        <ParticlesBackground />
    </PageTemplate>
  );
}

export default AnagramPage;