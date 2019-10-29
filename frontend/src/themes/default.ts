const availableAccents = [
  { foreground: ['#eab6fd'], background: ['#1d0922'] },
  { foreground: ['#01e5ac'], background: ['#1d01da'] },
  { foreground: ['#738c74'], background: ['#111059'] },
  { foreground: ['#57bc56'], background: ['#004231'] },
  { foreground: ['#52a772'], background: ['#3b1765'] },
  { foreground: ['#03a0b8'], background: ['#05112e'] },
  { foreground: ['#2cf22c'], background: ['#1d2edb'] },
  { foreground: ['#9ea356'], background: ['#072658'] },
  { foreground: ['#8ba270'], background: ['#0c0d26'] },
  { foreground: ['#b17375'], background: ['#070456'] },
  { foreground: ['#28d3ae'], background: ['#2f0e03'] },
  { foreground: ['#1190c8'], background: ['#1b0606'] },
  { foreground: ['#b5dbc8'], background: ['#6c01a3'] },
  { foreground: ['#f9733f'], background: ['#112739'] },
  { foreground: ['#e82f0f'], background: ['#0f0508'] },
];

type Theme = {
  palette: {
    foreground: string[],
    background: string[],
    grayscale: string[],
  },
  fonts: {
    primary: string,
    pre: string,
    quote: string,
    sizes: {
      big: string,
      normal: string,
    }
  },
  sizes: {
    maxWidth: string,
    spacer: number,
  }
};

const theme: Theme = {
  palette: {
    ...availableAccents[Math.floor(Math.random() * availableAccents.length)],
    grayscale: [
      '#212121',
      '#414141',
      '#616161',
      '#9e9e9e',
      '#bdbdbd',
      '#e0e0e0',
      '#eeeeee',
      '#ffffff',
    ],
  },
  fonts: {
    primary: 'Helvetica Neue, Helvetica, Roboto, sans-serif',
    pre: 'Consolas, Liberation Mono, Menlo, Courier, monospace',
    quote: 'Georgia, serif',
    sizes: {
      big: '2rem',
      normal: '14px',
    },
  },
  sizes: {
    maxWidth: '1100px',
    spacer: 8,
  },
}


export default theme