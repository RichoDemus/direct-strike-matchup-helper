<script>
    import {selectedCommanderStore} from "./stores";

    export let commander;

    function setSelectedCommander() {
        selectedCommanderStore.set(commander);
    }
</script>

<style>

</style>

<div>
    <div on:click={setSelectedCommander} on:keypress={setSelectedCommander} role="button" tabindex="0">
        <img src="{commander}.png" alt={commander}/>
        <span>{commander}</span>
    </div>

</div>