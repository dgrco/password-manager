<script>
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
  let masterPass = "";
  let safeName = "";
  let createDisabled = true;
  let status = "not_created";

  function updateCreateStatus() {
    createDisabled =
      masterPass.length > 0 && safeName.length > 0 ? false : true;
  }

  async function createDBFile() {
    safeName = safeName + ".psdb";
    status = await invoke("create_db", {db_name: safeName, master_password: masterPass});
  }
</script>

<button on:click={() => goto("/")}>Back</button>
<div class="container">
  <h1>Create a New SafePass DB File!</h1>

  <div>
    <div class="form-group">
      <label for="database-name">Safe Name:</label>
      <input
        bind:value={safeName}
        type="text"
        id="database-name"
        placeholder="Awesome safe name..."
        on:input={updateCreateStatus}
      />
    </div>

    <div class="form-group">
      <label for="master-password">Master Password:</label>
      <input
        bind:value={masterPass}
        type="password"
        id="master-password"
        placeholder="Super secret master password..."
        on:input={updateCreateStatus}
      />
    </div>

    <div class="create-btn">
      <button disabled={createDisabled} on:click={createDBFile}>Create</button>
    </div>
  </div>
  {status}
</div>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0 13vw;
    height: 75vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: left;
  }

  h1 {
    text-align: center;
    margin-bottom: 1.5em;
  }

  .create-btn {
    display: flex;
    justify-content: center;
  }

  .form-group {
    margin-bottom: 1rem;
  }

  .form-group label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: bold;
  }

  .form-group input {
    width: 100%;
    box-sizing: border-box;
  }

  input,
  button {
    text-decoration: none;
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    margin: 10px 10px;
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }
  button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  input,
  button {
    outline: none;
  }

  button:disabled {
    background-color: lightgrey;
  }
  button:disabled:hover {
    border-color: transparent;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    input,
    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
    button:active {
      background-color: #0f0f0f69;
    }
  }
</style>
