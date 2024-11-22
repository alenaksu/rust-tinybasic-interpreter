import init, { Interpreter } from '../../out/tinybasic';

const terminal = document.querySelector<HTMLDivElement>('#terminal')!;
const input = document.querySelector<HTMLInputElement>('#input')!;
const output = document.querySelector<HTMLDivElement>('#output')!;
let interpreter: Interpreter;

const write = (text: string) => {
    output.innerText += text;
    output.scrollTop = output.scrollHeight;
};

const clear = () => {
    output.innerText = '';
};

const readLine = () =>
    new Promise<string>((resolve) => {
        const controller = new AbortController();
        input.addEventListener(
            'keydown',
            (e) => {
                if (e.key === 'Enter') {
                    const value = input.innerText.trim().toUpperCase();
                    input.innerText = '';

                    controller.abort();
                    resolve(value);
                }
            },
            { signal: controller.signal }
        );
    });

const setPrompt = (prompt: string) => {
    input.dataset.prompt = prompt;
};

const loadFile = async () => {
    const [fileHandle] = await window.showOpenFilePicker({
        types: [
            {
                description: 'BASIC Program',
                accept: {
                    'text/plain': ['.bas']
                }
            }
        ]
    });

    const file = await fileHandle.getFile();
    const contents = await file.text();

    return contents;
};

const saveFile = async (source: string) => {
    const fileHandle = await window.showSaveFilePicker({
        types: [
            {
                description: 'BASIC Program',
                accept: {
                    'text/plain': ['.bas']
                }
            }
        ]
    });

    const writable = await fileHandle.createWritable();
    await writable.write(source);
    await writable.close();
};

(window as any).terminal = {
    terminal_write: write,
    terminal_read_line: readLine,
    terminal_clear: clear,
    terminal_set_prompt: setPrompt
};

(window as any).io = {
    io_load_file: loadFile,
    io_save_file: saveFile
};

clear();
init().then(() => {
    terminal.addEventListener('focus', () => {
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
        }
    });

    interpreter = new Interpreter();
    interpreter.execute().then(() => {
        write('BYE.');
    });
});
