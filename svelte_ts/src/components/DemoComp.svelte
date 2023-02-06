<script lang="ts">
    import { onMount, afterUpdate, getContext } from 'svelte';
    import { sumStore } from '../stores.js';

    // Properties are just exports
    export let nr1: number = undefined;
    export let nr2: number = undefined;

    // Lifecycles
    onMount(() =>
      console.log("DemoComp frist rendered to DOM")
    );
    afterUpdate(() =>
        console.log("DemoComp updated")
    );

    // reading from global context
    let car = getContext('car');
</script>

<div>
    <ul>
        <li>Number 1 is: <code>{nr1}</code></li>
        <li>Number 2 is: <code>{nr2}</code></li>
<!-- $storeVal = subscribe() & unsubscribe() on destroy = no boilerplate! -->
        <li>Sum Store: <code>{$sumStore}</code></li>
<!-- Logic blocks start with {# continue with {:else -->
        {#if nr1 == nr2}
            <li>AI says: Same numbers!</li>
        {/if}
    </ul>
    <br>
    BTW - Here is the car color from context: {car.color}
</div>

<style>
    /* unused CSS example - is not bundled -> warning during build */
    div {
        font-size: 120%;
    }
</style>