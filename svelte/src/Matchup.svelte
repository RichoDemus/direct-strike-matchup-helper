<script>

    export let commander;
    export let opponent;
    export let outcome;
    export let strategy;
    console.log(strategy)
</script>
<style>
    .container {
        border: solid;
    }
    .item-3 {
        white-space: pre-line
    }
</style>
<div>
    <div class="container">
        <div class="item item-1"><h3>{commander} vs {opponent}</h3></div>
        <div class="item item-2">{outcome}</div>
        <div class="item item-3">{strategy}</div>
    </div>
</div>