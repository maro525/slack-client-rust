<script lang="ts">
  import "carbon-components-svelte/css/white.css";

  import {
    OrderedList,
    ListItem,
    Button,
    TextInput,
    Loading,
  } from "carbon-components-svelte";
  import ChevronRight from "carbon-icons-svelte/lib/ChevronRight.svelte";

  import { invoke } from "@tauri-apps/api/tauri";

  import type { Message, Response } from "~/model/search-messages";

  let messagesPromise: Promise<Message[]> = Promise.resolve([]);
  let query = "";

  const handleClickUpdate = () => {
    console.log("handle click");
    messagesPromise = invoke<Response>("search_messages", { query }).then(
      (r) => {
        console.log(r);
        return r.messages;
      }
    );
  };
</script>

<main>
  <OrderedList>
    <TextInput labelText="Search" bind:value={query} />
    <Button on:click={handleClickUpdate}>Search</Button>
    {#await messagesPromise}
      <Loading />
    {:then messages}
      {#each messages as message}
        <ListItem>
          <ChevronRight /> <span>{message}</span>
        </ListItem>
      {/each}
    {:catch error}
      <ListItem>
        <p>{error}</p>
      </ListItem>
    {/await}
  </OrderedList>
</main>

<style>
  main {
    display: flex;
    margin: 15px;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
