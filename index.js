import init, { choose_page } from './jbaublitz_me_wasm.js'

function fadeOut(page) {
    var fio = document.getElementById("fadeinout");
    fio.className = "body-panel-out";
    fio.addEventListener('animationend', function() {
        wasm_bindgen.choose_page(page);
        this.className = "body-panel-in";
    });
}

async function run() {
    await init();
    choose_page('index');
    var fio = document.getElementById("fadeinout");
    fio.className = "body-panel-in";
    fio.removeAttribute('hidden');
}

run();
