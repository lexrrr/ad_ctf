<script lang="ts">
    /** @type {import('./$types').PageData} */
    // import {Card} from 'flowbite-svelte';
    import { Card, Timeline, TimelineItem, Button } from 'flowbite-svelte';
    import { ArrowRightOutline } from 'flowbite-svelte-icons';
    export let data;

    $: sortedNotifications = [...data.notifications].sort((a, b) => new Date(b.receivedAt).getTime() - new Date(a.receivedAt).getTime());

    function formatDate(isoString: string) {
        const date = new Date(isoString);
        return date.toLocaleString('default', { month: 'long', hour: '2-digit', minute: '2-digit' });
    }
</script>

<div class="flex items-center flex-col">
    <h1 class="text-5xl mt-24 mb-20">Received Notifications</h1>
    <Timeline id="notifications" class="w-2/5">
        {#each sortedNotifications as notification (notification.id)}
            <TimelineItem date={formatDate(notification.receivedAt)}>
                <p class="mb-4 text-base font-normal">{notification.message}</p>
            </TimelineItem>
        {/each}
    </Timeline>
</div>
