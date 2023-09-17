<script>

    import {selectedCommanderStore} from "./stores";
    import Matchup from "./Matchup.svelte";

    export let matchups;

</script>

<style>
</style>

<div>
    <span>Matchups for {$selectedCommanderStore}</span><br/>
    {#each matchups.get($selectedCommanderStore).toSorted((a,b) => a.opponent.localeCompare(b.opponent)) as matchup}
        <Matchup commander={$selectedCommanderStore} opponent={matchup.opponent} outcome={matchup.outcome}
                 strategy={matchup.strategy}/>
    {/each}
</div>
