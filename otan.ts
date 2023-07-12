import * as readline from 'readline';

function Otan(): void {
  const array: { [key: string]: string } = {
    ' ': ' ',
    A: 'Alpha',
    B: 'Bravo',
    C: 'Charlie',
    D: 'Delta',
    E: 'Echo',
    F: 'Foxtrot',
    G: 'Golf',
    H: 'Hotel',
    I: 'India',
    J: 'Juliet',
    K: 'Kilo',
    L: 'Lima',
    M: 'Mike',
    N: 'November',
    O: 'Oscar',
    P: 'Papa',
    Q: 'Quebec',
    R: 'Romeo',
    S: 'Sierra',
    T: 'Tango',
    U: 'Uniform',
    V: 'Victor',
    W: 'Whiskey',
    X: 'Xray',
    Y: 'Yankee',
    Z: 'Zulu',
  };

  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  rl.question('Ecrivez le texte à transcrire en OTAN : ', (otanInput) => {
    rl.close();

    const translate: string[] = otanInput.split('').map((char) => array[char.toUpperCase()] ?? char);
    console.log('\n' + translate.join(' ') + '\n');
  });
}

Otan();
