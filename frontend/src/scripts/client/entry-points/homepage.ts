import { StoredCredential } from "../models/StoredCredential.ts";

const init = async () => {
  const $form = document.getElementById("app-homepage-form");

  if (!$form) {
    console.error("Webterm: Form not found");
    return;
  }

  $form.addEventListener("submit", async (e) => {
    e.preventDefault();
    const $serverId = $form.querySelector("input[name='server-id']") as HTMLInputElement;
    const $password = $form.querySelector("input[name='password']") as HTMLInputElement;
    const serverId = $serverId.value;
    const serverPassword = $password.value;

    const { index, secretKey } = await StoredCredential.store(serverId, serverPassword);

    window.location.href = `/terminal?store_index=${index}&store_key=${secretKey}`;
  });
};

await init();
