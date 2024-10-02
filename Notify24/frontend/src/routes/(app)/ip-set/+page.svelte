<script lang="ts">
    /** @type {import('./$types').PageData} */
    import { ButtonGroup, Helper, Badge, Button, Textarea, Label, InputAddon, Modal, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell, TableSearch, Input } from 'flowbite-svelte';
    import { CloseCircleOutline } from 'flowbite-svelte-icons';
    import { enhance } from '$app/forms';
    import type { ActionData } from "../../../../.svelte-kit/types/src/routes/(app)/ip-set/$types";
    import {goto} from "$app/navigation";
    export let data;

    /** @type {import('./$types').ActionData} */
    export let form: ActionData;
    let ipInputValue = '';
    let inputHelperMessage = "Please enter a valid IP address";
    let inputHelperColor: "red" | "green" = "red";
    let searchTerm = '';
    let addIpWarning = '';

    $: sets = data.ipSets;
    if (form) {
        sets = form.ipSets;
    } else {
        sets = data.ipSets;
    }

    $: filteredItems = sets.filter((item) => item.name.toLowerCase().indexOf(searchTerm.toLowerCase()) !== -1);


    let defaultModal = false;

    let idCount = 0;
    let selectedIps = [];

    const handleSend = (id: string) => {
        goto(`/send?ipSetId=${id}`)
    }
    const handleClose = (ipId) => {
        selectedIps = selectedIps.filter((ip) => ip.id !== ipId);
    };

    function handleInput(event) {
        if (event.key === 'Enter') {
            event.preventDefault();
            addIpToSet()
        }
    }

    function addIpToSet() {
        addIpWarning = '';
        selectedIps.push({ id: idCount++, ip: ipInputValue });
        selectedIps = selectedIps
        ipInputValue = '';
    }

    const validateIpInput = (event) => {
        let ip = event.target.value
        const ipRegex = /([\d]{1,3}\.[\d]{1,3}\.[\d]{1,3}\.[\d]{1,3}):([\d]{1,5})/
        if (ipRegex.test(ip)) {
            inputHelperMessage = "Confirm with enter to add IP to the list";
            inputHelperColor = "green";
            return true;
        } else {
            inputHelperMessage = "Please enter a valid IP address";
            inputHelperColor = "red";
            return false;
        }
    }

    function handleIpInputButtonClick() {
        addIpToSet()
    }
</script>

{#if sets.length === 0}
    <div class="flex justify-center mt-36 mb-4">
        <h1 class="text-3xl">You haven't created any IP Sets yet</h1>
    </div>
{/if}
<div class="flex justify-center m-5">
    <Button on:click={() => (defaultModal = true)}>Create IP Set</Button>
</div>

{#if sets.length !== 0}
<TableSearch placeholder="Search by IP Set name" hoverable={true} bind:inputValue={searchTerm}>
    <TableHead>
        <TableHeadCell>ID</TableHeadCell>
        <TableHeadCell>Name</TableHeadCell>
        <TableHeadCell>Description</TableHeadCell>
        <TableHeadCell>IPs</TableHeadCell>
        <TableHeadCell></TableHeadCell>
    </TableHead>
    <TableBody tableBodyClass="divide-y">
        {#each filteredItems as item}
            <TableBodyRow>
                <TableBodyCell>{item.id}</TableBodyCell>
                <TableBodyCell>{item.name}</TableBodyCell>
                <TableBodyCell>{item.description}</TableBodyCell>
                <TableBodyCell>{item.ips}</TableBodyCell>
                <TableBodyCell><Button outline size="sm" color="blue" on:click={() => handleSend(item.id)}>Send Notification</Button></TableBodyCell>
            </TableBodyRow>
        {/each}
    </TableBody>
</TableSearch>
{/if}

<Modal title="Add IP Set" bind:open={defaultModal}  class="min-w-full">
    <form method="POST" action="/ip-set?/create"
        use:enhance={({ formData, cancel }) => {
            if (selectedIps.length === 0) {
                addIpWarning = "Please add at least one IP address to the list";
                cancel()
                return;
            }
            defaultModal = false;
            formData.append('ips', selectedIps.map((ipObject) => ipObject.ip).join(";"))
            selectedIps = [];
    }}>
        <div class="grid gap-4 mb-4 sm:grid-cols-2">
            <div>
                <Label for="name" class="mb-2">Name</Label>
                <Input type="text" id="name" name="name" placeholder="Type IP Set name" required />
            </div>
            <div>
            </div>


            <div class="col-span-2">
                {#each selectedIps as ip (ip.id)}
                    <Badge class="m-0.5" dismissable large>
                        {ip.ip}
                        <button slot="close-button" on:click={() => handleClose(ip.id)} type="button" class="ml-1" aria-label="Remove">
                            <CloseCircleOutline class="h-4 w-4" />
                            <span class="sr-only">Remove badge</span>
                        </button>
                    </Badge>
                {/each}

                <Label for="ip" class="mb-2">IP Address with Port (You can add more than one)</Label>
                <ButtonGroup class="w-full">
                <Input type="text" id="ip" placeholder="127.0.0.0:6060" color={inputHelperColor} bind:value={ipInputValue} on:input={validateIpInput} on:keypress={handleInput}/>
                <Button color={inputHelperColor} on:click={handleIpInputButtonClick}>Add</Button>
                </ButtonGroup>
                <Helper class="mt-2" color={inputHelperColor}>
                    <span class="font-medium">{inputHelperMessage}</span>
                </Helper>
            </div>
            <div class="sm:col-span-2">
                <Label for="description" class="mb-2">Description</Label>
                <Textarea id="description" placeholder="IP Set description here" rows="4" name="description" required />
            </div>
            <div>
                <Helper class="my-2" color="red">
                    <span class="font-medium">{addIpWarning}</span>
                </Helper>
                <Button type="submit"  class="w-52" >
                    <svg class="mr-1 -ml-1 w-6 h-6" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M10 5a1 1 0 011 1v3h3a1 1 0 110 2h-3v3a1 1 0 11-2 0v-3H6a1 1 0 110-2h3V6a1 1 0 011-1z" clip-rule="evenodd" /></svg>
                    Add new IP Set
                </Button>
            </div>
        </div>
    </form>
</Modal>
