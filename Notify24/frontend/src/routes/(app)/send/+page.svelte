<script>
import { page } from '$app/stores';
import {Button, Card, Textarea} from 'flowbite-svelte';
import { enhance } from '$app/forms';
import { Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from 'flowbite-svelte';

/** @type {import('./$types').PageData} */
export let data;

export let form;

let ipSetName = $page.url.searchParams.get('ipSetId');


</script>
<div class="flex n-w-1/2 justify-center items-center flex-col">


    <Card class="min-w-1/2 my-20">
        <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">Send a Notification</h5>
        <p class="mb-3 font-normal text-gray-700 dark:text-gray-400 leading-tight">Send a message to the IP set: {data.name}</p>
        <p class="mb-3 font-normal text-gray-700 dark:text-gray-400 leading-tight"> With the IPs: {data.ips.join(' ')}</p>
        <form method="POST" action="/send?/send" use:enhance={({ formData }) => {
            formData.append('ipSetId', data.id)
        }}>
            <Textarea class="my-5" id="notification" placeholder="Your notification here" rows="4" name="notification" required />
            <Button type="submit" class="w-52">
                Send Notification
            </Button>
        </form>
    </Card>
</div>
<div class="flex justify-center flex-row m-5">
    {#if form?.response}
        <Table class="w-1/2 ">
            <TableHead>
                <TableHeadCell>Recipient</TableHeadCell>
                <TableHeadCell>UUI</TableHeadCell>
                <TableHeadCell>Status</TableHeadCell>
            </TableHead>
            <TableBody tableBodyClass="divide-y">
                {#each form.response as notificationResponse}
                    <TableBodyRow>
                        <TableBodyCell>{notificationResponse.recipient}</TableBodyCell>
                        <TableBodyCell> {notificationResponse.uuid}</TableBodyCell>
                        <TableBodyCell> {notificationResponse.status}</TableBodyCell>
                    </TableBodyRow>
                {/each}
            </TableBody>
        </Table>
    {/if}
</div>

