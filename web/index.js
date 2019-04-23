if (document.readyState == 'loading') {
    document.addEventListener('DOMContentLoaded', ready)
} else {
    ready();
}
var body = "[1] $ ||X|| X X\n[3] $ 1 ||X|| X\n[0] $ 1 3 ||X||\n[3] # |1| _3_ 0\n[5] $ ||1|| 3 0\n[6] $ 5 ||3|| 0\n[3] $ 5 3 ||0||\n";
var out = "5 6 3"
addOutput("FIFO", 1, 6, body, out);

var pageNumberChangedByUser = false;

function ready() {
    document.getElementById("clear-button").addEventListener('click', clearOutputs);
    document.getElementById('start-button').addEventListener('click', startSimulation);
    document.getElementById('refs-box').addEventListener('change', checkRefs);
    document.getElementById('page-box').addEventListener('change', checkPage);
    document.getElementById('gen-refs-btn').addEventListener('click', generateRefs);
    document.getElementById('gen-page-btn').addEventListener('click', generatePages);
    document.getElementById('ref-num-box').addEventListener('change', checkRefsNum);
    let closeButtons = document.getElementsByClassName('btn-close');
    for (let i = 0; i < closeButtons.length; i++) {
        closeButtons[i].addEventListener('click', closeOutputBox);
    }
}

function closeOutputBox(event) {
    event.target.parentElement.remove();
}

function clearOutputs() {
    let outputs = document.getElementsByClassName('output-field')[0];
    while (outputs.hasChildNodes) {
        outputs.removeChild(outputs.firstChild);
    }
}

function startSimulation() {
    let selectedAlgorithm = document.getElementById('alg-select');
    let algorithm = selectedAlgorithm.options[selectedAlgorithm.selectedIndex].value;
    if (algorithm === 'none') {
        alert('Please choose the algorithm first');
        return;
    }
    let request = "http://localhost:8001/" + algorithm;
    let retJson;
    let xhttp = new XMLHttpRequest();

    xhttp.onreadystatechange = function() {
        if(this.readyState == 4 && this.status == 200) {
            retJson = JSON.parse(this.responseText);
            addOutput(algorithm.toUpperCase(), retJson.hits, retJson.misses, retJson.body, retJson.output_stage);
        }
    };

    let pageNum = document.getElementById('page-box').value;
    let references = document.getElementById('refs-box').value;

    let json = {pages: 1, requests: ''};
    json.pages = parseInt(pageNum);
    json.requests = references;

    xhttp.open("POST", request, true);
    xhttp.send(JSON.stringify(json));


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

function checkRefsNum(event) {
    let numBox = event.target;
    pageNumberChangedByUser = true;
    if (numBox.value <= 0) {
        numBox.value = 1;
    }
}

function generateRefs() {
    let refs = "";
    let n = 0;
    let refsBox = document.getElementById('refs-box');
    if (pageNumberChangedByUser) {
        n = document.getElementById('ref-num-box').value;
        pageNumberChangedByUser = false;
    } else {
        n = Math.floor(Math.random() * 200) + 1;
        document.getElementById('ref-num-box').value = n;
    }

    for (let i = 0; i < n; i++) {
        refs += Math.floor(Math.random() * 10);
        refs += ' ';
    }

    refsBox.value = refs.trim();
}

function generatePages() {
    let pageBox = document.getElementById('page-box');
    let val = pageBox.value;
    while (val == pageBox.value) {
        pageBox.value = Math.floor(Math.random() * 10) + 1;
    }
}

function addOutput(algorithmName, hits, misses, bodyStr, resultStr) {
    let outputBox = document.createElement('div');
    outputBox.classList.add('output-box');

    let closeButton = document.createElement('BUTTON');
    closeButton.classList.add('btn', 'btn-close');
    closeButton.innerText = '+';
    closeButton.addEventListener('click', closeOutputBox);
    outputBox.append(closeButton);

    let header = outputHeader(algorithmName, hits, misses);
    outputBox.append(header);

    let result = outputFinal(resultStr);
    outputBox.append(result);

    let section = outputBody(bodyStr);
    outputBox.append(section);


    document.getElementById('outputs').append(outputBox);
}

function outputHeader(name, hits, misses) {
    let retHeader = document.createElement('div');
    retHeader.classList.add('output-header-box');
    let content = `
        <strong>${name}</strong> <br>
        Page hits: <strong>${hits}</strong> <br>
        Page misses: <strong>${misses}</strong> <br>
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

    for (let i = 0; i < nodes.length; i++) {
        // in case that .trim() do not work
        // if (nodes[i] != "") {
            outputSection.append(makeNode(nodes[i]));
        // }
    }

    return outputSection;
}

function makeNode(nodeStr) {
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
