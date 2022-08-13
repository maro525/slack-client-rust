<script lang="ts">
  import "carbon-components-svelte/css/white.css";

  import {
    OrderedList,
    ListItem,
    Button,
    TextInput,
    Loading,
  } from "carbon-components-svelte";

  import type { Message } from "~/model/messages";
  import MessageCard from "./components/molecules/MessageCard.svelte";

  import { search, hello } from "~/service/tauri";

  let messagesPromise: Promise<Message[]> = Promise.resolve([]);
  let query = "";

  const handleClickUpdate = () => {
    messagesPromise = search(query).catch((err) => console.log(err));
  };

  const callHello = async () => {
    const res = await hello();
    console.log(res);
  };
</script>

<main>
  <OrderedList>
    <TextInput labelText="Search" bind:value={query} />
    <Button on:click={handleClickUpdate}>Search</Button>
    <Button on:click={callHello}>hello</Button>
    {#await messagesPromise}
      <Loading />
    {:then messages}
      {#each messages as message}
        <ListItem>
          <MessageCard {message} />
        </ListItem>
      {/each}
    {:catch error}
      <ListItem>
        <p style="color: red">{error}</p>
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
