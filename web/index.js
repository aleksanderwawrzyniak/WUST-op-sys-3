if (document.readyState == 'loading') {
    document.addEventListener('DOMContentLoaded', ready)
} else {
    ready();
}
var head = 'FIFO\nPage hits: 1\nPage misses: 6';
var body = "[1] $ ||X|| X X\n[3] $ 1 ||X|| X\n[0] $ 1 3 ||X||\n[3] # |1| _3_ 0\n[5] $ ||1|| 3 0\n[6] $ 5 ||3|| 0\n[3] $ 5 3 ||0||\n";
var out = "5 6 3"
addOutput(head, body, out);

function ready() {
    document.getElementById("clear-button").addEventListener('click', clearOutputs);
    document.getElementById('start-button').addEventListener('click', startEmulation);
    document.getElementById('refs-box').addEventListener('change', checkRefs);
    document.getElementById('page-box').addEventListener('change', checkPage);
    document.getElementById('gen-refs-btn').addEventListener('click', generateRefs);
    document.getElementById('gen-page-btn').addEventListener('click', generatePages);
}


function clearOutputs() {
    let outputs = document.getElementsByClassName('output-field')[0];
    while (outputs.hasChildNodes) {
        outputs.removeChild(outputs.firstChild);
    }
}

function startEmulation() {
    let selectedAlgorithm = document.getElementById('alg-select');
    let algorithm = selectedAlgorithm.options[selectedAlgorithm.selectedIndex].value;
    switch (algorithm) {
        case "none":
            alert("Please choose the algorithm to emulate before clicking START EMULATION button");
            break;
        case "fifo":
            //todo
            console.log(algorithm + " being emulated");
            break;
        case "lru":
            //todo
            console.log(algorithm + " being emulated");
            break;
        case "alru":
            //todo
            console.log(algorithm + " being emulated");
            break;
        case "opt":
            //todo
            console.log(algorithm + " being emulated");
            break;
        case "rand":
            //todo
            console.log(algorithm + " being emulated");
            break;
    }
}

function checkRefs(event) {
    let refsBox = event.target;
    refsBox.value = refsBox.value.replace(/[^\d\s:]/g, '').replace(/\s{2,}/g, ' ');
}

function checkPage(event) {
    let pageBox = event.target;
    if (pageBox.value <= 0) {
        pageBox.value = 1;
    }
}

function generateRefs() {
    let refs = "";
    let refsBox = document.getElementById('refs-box');
    let n = Math.floor(Math.random() * 200) + 1;

    for (let i = 0; i < n; i++) {
        refs += Math.floor(Math.random() * 10);
        refs += ' ';
    }

    refsBox.value = refs;
}

function generatePages() {
    let pageBox = document.getElementById('page-box');
    let val = pageBox.value;
    while (val == pageBox.value) {
        pageBox.value = Math.floor(Math.random() * 10) + 1;
    }
}

function addOutput(headerStr, bodyStr, resultStr) {
    let outputBox = document.createElement('div');
    outputBox.classList.add('output-box');
    let header = outputHeader(headerStr);
    console.log(header);
    outputBox.append(header);

    let result = outputFinal(resultStr);
    outputBox.append(result);

    let section = outputBody(bodyStr);
    outputBox.append(section);


    document.getElementById('outputs').append(outputBox);
}

function outputHeader(header) {
    let retHeader = document.createElement('div');
    retHeader.classList.add('output-header-box');
    let headerParts = header.split('\n');
    let algName = headerParts[0];
    let pagesHit = headerParts[1].replace(/\D/g, '');
    let pagesMiss = headerParts[2].replace(/\D/g, '');
    let content = `
        <strong>${algName}</strong> <br>
        Page hits: <strong>${pagesHit}</strong> <br>
        Page misses: <strong>${pagesMiss}</strong> <br>
    `
    retHeader.innerHTML = content;
    return retHeader;
}

function outputFinal(result) {
    let finalNode = document.createElement('div');
    finalNode.classList.add('output-node', 'final-node');
    let pages = result.split(' ');

    for (let i = 0; i < pages.length; i++) {
        let page = document.createElement('span');
        page.classList.add('page');
        page.innerText = pages[i];
        finalNode.append(page);
    }

    return finalNode;
}

function outputBody(body) {
    let outputSection = document.createElement('div');
    outputSection.classList.add('output-section');
    let nodes = body.trim().split('\n');
    console.log(nodes);

    for (let i = 0; i < nodes.length; i++) {
        // in case that .trim() do not work
        // if (nodes[i] != "") {
            outputSection.append(makeNode(nodes[i]));
        // }
    }

    return outputSection;
}

function makeNode(nodeStr) {
    console.log(nodeStr);
    let node = document.createElement('div');
    node.classList.add('output-node');

    let splitted = nodeStr.split(' ');

    let ref = document.createElement('span');
    ref.classList.add('page', 'ref');
    ref.innerText = splitted[0].replace(/\D/g, '');
    node.append(ref);

    let arrow = document.createElement('span');
    arrow.classList.add('page', 'arrow');
    arrow.innerText = '=>';
    node.append(arrow);

    if (splitted[1] == '$') {
        node.classList.add('output-miss-node');
    }

    for (let i = 2; i < splitted.length; i++) {
        node.append(makePage(splitted[i]));
    }

    return node;
}

function makePage(str) {
    let page = document.createElement('span');
    page.classList.add('page');

    // ||x||
    if (/\|\|(X|\d+)\|\|/.test(str)) {
        page.classList.add('miss-page');
        page.innerText = str.replace(/[^X\d]/g, '');
        return page;
    }

    // _|x|_
    if (/_\|(X|\d+)\|_/.test(str)) {
        page.classList.add('hit-aim-page');
        page.innerText = str.replace(/[^X\d]/g, '');
        return page;
    }

    // |x|
    if (/\|(X|\d+)\|/.test(str)) {
        page.classList.add('aim-page');
        page.innerText = str.replace(/[^X\d]/g, '');
        return page;
    }

    // _x_
    if (/_(X|\d+)_/.test(str)) {
        page.classList.add('hit-page');
        page.innerText = str.replace(/[^X\d]/g, '');
        return page;
    }

    // x
    if (/(X|\d+)/.test(str)) {
        page.innerText = str.replace(/[^X\d]/g, '');
        return page;
    }
}
