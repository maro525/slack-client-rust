<script lang="ts">
  import { Tile, Button } from "carbon-components-svelte";
  import type { Message } from "~/model/search-messages";
  import Launch from "carbon-icons-svelte/lib/Launch.svelte";

  export let message: Message;
</script>

<Tile>
  <b>{message.user_name}</b>
  <span>{message.created_at}</span>
  <pre style="white-space: pre-wrap">{message.text}</pre>
  <Button
    iconDescription="popup"
    icon={Launch}
    on:click={() => {
      window.open(message.permalink);
    }}>open_in_new</Button
  >
</Tile>
