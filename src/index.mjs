import { h, render } from 'https://esm.sh/preact';
import htm from 'https://esm.sh/htm'; 

const html = htm.bind(h);

function App(props){
    return html`
        <div>
        ${props.cpus.map((cpu) =>{
            return html`<div>${cpu.toFixed(2)}% usage</div>`
        })}
        </div>
    `;
}



let i = 0;
setInterval(async () => {
    let response = await fetch('/api/cpus');
    if(response.status !== 200){
        throw new Error(`Http error: ${response.status}`);
    }
    let json = await response.json();
    
    const app = h('pre',null,JSON.stringify(json,null,2));
    render(html`<${App} cpus=${json}></${App}>`,document.body);
    
},1000);
