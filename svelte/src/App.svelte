<script>
    import {onMount} from "svelte";
    import {get_matchups, throwException} from "./call_wasm";
    import Commander from "./Commander.svelte";
    import Matchups from "./Matchups.svelte";
    import {matchupsStore, selectedCommanderStore} from "./stores";

    let commanders = [];

    matchupsStore.subscribe(value => {
        if (value) {
            console.log("value type:", typeof value);
            console.log("value: ", value);
            if (typeof value.keys === "function") {
                commanders = Array.from(value.keys());
                commanders.sort();
            }
        }
    })

    onMount(async () => {

        let matchups = await get_matchups();
        console.log("matchups type :", typeof matchups);
        console.log("commanders:", Array.from(matchups.keys()));
        console.log("matchups:", matchups);
        // console.log("matcjups:", JSON.stringify(matchups));
        matchupsStore.set(matchups);

    });

    async function click() {
        const f = await throwException();
        try {
            f();
        } catch (e) {
            alert("rust code returned Err(): " + e);
        }
    }

    function unselectCommander() {
        selectedCommanderStore.set(null);
    }

    let selectedCommander = null;
    selectedCommanderStore.subscribe((value) => {
        selectedCommander = value;
    });
</script>

<style>
    /*h1 {*/
    /*    color: purple;*/
    /*}*/
</style>
<div>
    {#if selectedCommander }
        <button on:click={unselectCommander}>
            Back
        </button>
        <Matchups commander={selectedCommander} commanders={commanders} matchups={$matchupsStore}/>
    {:else}
        {#each commanders as commander}
            <Commander commander={commander}/>
        {/each}
    {/if}
</div>
