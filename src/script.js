
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
        const response = await fetch(`/subtract/${num1}/${num2}`);
        const result = await response.text();
        document.getElementById("resultado").textContent = result;
    }
}

function init() {
    document.getElementById("btnSumar").addEventListener("click", sumar);
    document.getElementById("btnRestar").addEventListener("click", restar);
}

document.addEventListener("DOMContentLoaded", init);