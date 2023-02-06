<script lang="ts">
    import { setContext, onMount } from 'svelte';
    import { sumStore } from './stores.js';
    import {Content, Tag, TextInput} from 'carbon-components-svelte';
import {
    StructuredList,
    StructuredListHead,
    StructuredListRow,
    StructuredListCell,
    StructuredListBody,
} from "carbon-components-svelte";
import { Tile } from "carbon-components-svelte";
import { Button } from "carbon-components-svelte";

import DemoComp from "./components/DemoComp.svelte";
    import {add, Car, js_alert, js_alert_number} from "../rust-crate/pkg";
    import FileParser from "./components/FileParser.svelte";
    import {parse} from "../rust-crate/pkg/rust-crate.js";


let car = Car.new();
car.number = 5;
car.color = 775577;
 // Whenever you’re confused about what properties are accessible from JavaScript, console.log that object and open that object’s prototype description.
console.log(car); // we see also the WASM ptr
let car2 = car.duplicate();
console.log(car2);
setContext('car', car);

let nr1: number = 4;
let nr2: number = 5;
let sum: number;
let demoCompRef;

/**
 * Reactivity:
 * update variable / run this code on every change of the vars on the right
 * Important: Reactivity only if directly assigned to variable!
 */
$: sumFormula = `${nr1} + ${nr2} = ${sum}`;
$: {sumStore.set(sum); console.log(`Stored ${sum}`);}
$: console.log(`The sum is ${sum}`);

onMount(() => {
    sumNumbers();
    // Oh, that works? -> null-safe access with plain JS!
    console.log('DemoComp props (with index): ' + JSON.stringify(demoCompRef?.$$.props));
})

function alertClick() {
    js_alert();
}

function alertNrClick() {
    js_alert_number();
}

function sumNumbers() {
    sum =add(nr1, nr2);
}
</script>

<!--
- No template - just start with html tag
- Just {} for binding
-->
<Content>
    <h1 style="text-align: center">
        Rust <a href="https://svelte.dev" target="_blank" rel="noreferrer">
        <img src='WASM.png' class="logo svelte" alt="Svelte Logo" height="35px"/></a>SM <a href="https://svelte.dev" target="_blank" rel="noreferrer">
        <img src='svelte.svg' class="logo svelte" alt="Svelte Logo" /></a>velte Showcase
    </h1>

    <br><h2>
        Car instance
    </h2>
    <Tag type="outline">WASM object access</Tag>
    <Tile light>
        <StructuredList condensed>
            <StructuredListHead>
                <StructuredListRow head>
                    <StructuredListCell head>Field</StructuredListCell>
                    <StructuredListCell head>Value</StructuredListCell>
                </StructuredListRow>
            </StructuredListHead>
            <StructuredListBody>
                <StructuredListRow>
                    <StructuredListCell>Color</StructuredListCell>
                    <StructuredListCell>{car.color}</StructuredListCell>
                </StructuredListRow>
                <StructuredListRow>
                    <StructuredListCell>Nr</StructuredListCell>
                    <StructuredListCell>{car.number}</StructuredListCell>
                </StructuredListRow>
                <StructuredListRow>
                    <StructuredListCell>Owner ID</StructuredListCell>
                    <StructuredListCell>{car.get_owner_id()}</StructuredListCell>
                </StructuredListRow>
            </StructuredListBody>
        </StructuredList>
    </Tile>

    <br><h3>Svelte → WASM → JS</h3>
    <Tag type="outline">JS alerts from WASM</Tag>
    <Tile>
        <Button on:click={alertClick}>Alert</Button>
        <Button kind="secondary" on:click={alertNrClick}>Alert Number</Button>
    </Tile>

    <br><h3>Add numbers</h3>
    <Tag type="outline">Reactivity | use WASM function</Tag>
    <Tile>
        <TextInput labelText="Number 1" placeholder="Enter a number" bind:value="{nr1}" on:keyup={sumNumbers}/> <!-- bind = 2-way binding -->
        <TextInput labelText="Number 2" placeholder="Enter a number" bind:value="{nr2}" on:keyup={sumNumbers}/>
        Added: {sumFormula}
    </Tile>
    <br>

    <br><h2>Demo Component</h2>
    <Tag type="outline">Lifecycle | passing responsive properties | global store | context</Tag>
    <Tile>
<!--  short prop assignment syntax + getting references to a component-->
    <DemoComp {nr1} {nr2} bind:this={demoCompRef}></DemoComp>
    </Tile>

    <br><h2>File parsing</h2>
    <Tag type="outline">Wasm function in component</Tag>
    <Tile>
        <FileParser parse={parse}/>
    </Tile>
</Content>