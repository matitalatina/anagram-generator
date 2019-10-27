import {get_anagram} from "anagram";

fetch('assets/110000_parole_italiane_con_nomi_propri.txt')
  .then(res => res.text())
  .then(txt => {
    console.log(get_anagram("Mattia Natali", txt.split('\n').filter(x => x.length > 4)))
  });