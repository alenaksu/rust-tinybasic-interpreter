import init, { Interpreter } from '../../out/tinybasic';

const CLEAR_SCRRN = '\x0C';

const terminanl = document.querySelector<HTMLDivElement>('#terminal')!;
const input = document.querySelector<HTMLInputElement>('#input')!;
const output = document.querySelector<HTMLDivElement>('#output')!;

const write = (text: string) => {
    output.innerHTML += `${text}\n`;
    output.scrollTop = output.scrollHeight;
};

const read = () => {
    const value = input.innerText.trim().toUpperCase();
    input.innerText = '';

    write(`:${value}`);

    return value;
};

const clear = () => {
    output.innerHTML = '';
};

clear();
init().then(() => {
    const interpreter = new Interpreter();

    terminanl.addEventListener('focus', () => {
        input.focus();
    });

    input.addEventListener('keydown', (e) => {
        switch (e.key) {
            case 'ArrowUp':
            case 'ArrowLeft':
            case 'ArrowRight':
            case 'ArrowDown': {
                e.preventDefault();
                break;
            }
            case 'Enter': {
                try {
                    const value = read();
                    const result = interpreter.execute(value);

                    if (result) {
                        if (result === CLEAR_SCRRN) {
                            clear();
                        } else {
                            write(`${result}\n`);
                        }
                    }
                } catch (e) {
                    write((e as Error).message);
                }
            }
        }
    });

    write('Ready!');
});
