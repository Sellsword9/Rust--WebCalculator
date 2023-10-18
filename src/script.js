
async function sumar() {
    const num1 = document.getElementById("num1").value;
    const num2 = document.getElementById("num2").value;

    if (num1 !== "" && num2 !== "") {
        const response = await fetch(`/add/${num1}/${num2}`);
        const result = await response.text();
        document.getElementById("resultado").textContent = result;
    }
}

async function restar() {
    const num1 = document.getElementById("num1").value;
    const num2 = document.getElementById("num2").value;

    if (num1 !== "" && num2 !== "") {
        const response = await fetch(`/diff/${num1}/${num2}`);
        const result = await response.text();
        document.getElementById("resultado").textContent = result;
    }
}

async function multiplicar() {
    const num1 = document.getElementById("num1").value;
    const num2 = document.getElementById("num2").value;

    if (num1 !== "" && num2 !== "") {
        const response = await fetch(`/multiply/${num1}/${num2}`);
        const result = await response.text();
        document.getElementById("resultado").textContent = result;
    }
}

async function dividir() {
    const num1 = document.getElementById("num1").value;
    const num2 = document.getElementById("num2").value;

    if (num1 !== "" && num2 !== "") {
        const response = await fetch(`/divide/${num1}/${num2}`);
        const result = await response.text();
        document.getElementById("resultado").textContent = result;
    }
}

async function potencia() {
    const num1 = document.getElementById("num1").value;
    const num2 = document.getElementById("num2").value;
    if (num1 > 20 || num2 > 20) {
        alert("20 es el valor máximo permitido para la operación de potencia, ninguno de los dos números puede ser mayor.");
    }else if (num1 !== "" && num2 !== "") {
        const response = await fetch(`/pow/${num1}/${num2}`);
        const result = await response.text();
        document.getElementById("resultado").textContent = result;
    }
}

async function raiz() {
    const num1 = document.getElementById("num1").value;
    const num2 = document.getElementById("num2").value;

    if (num1 !== "" && num2 !== "") {
        const response = await fetch(`/nrt/${num1}/${num2}`);
        const result = await response.text();
        document.getElementById("resultado").textContent = result;
    }
}
function init() {
    document.getElementById("btnSumar").addEventListener("click", sumar);
    document.getElementById("btnRestar").addEventListener("click", restar);
    document.getElementById("btnMultiplicar").addEventListener("click", multiplicar);
    document.getElementById("btnDividir").addEventListener("click", dividir);
    document.getElementById("btnPotencia").addEventListener("click", potencia);
    document.getElementById("btnRaiz").addEventListener("click", raiz);
}

document.addEventListener("DOMContentLoaded", init);