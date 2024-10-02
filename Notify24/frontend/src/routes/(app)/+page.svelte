<script>
    import {Button, GradientButton} from "flowbite-svelte";
    import {BrainOutline} from "flowbite-svelte-icons";
</script>

<div class="grid grid-cols-2 flex-grow gap-4  m-28 items-center justify-center">
    <div class="flex flex-col justify-center items-start p-4">
        <h1 class="text-4xl font-bold mb-2">Welcome to Your Decentralized Notification Network</h1>
        <h1 class="text-2xl font-bold mb-2">Seamless, Secure, and Autonomous Communication for Every Organization</h1>
        <p class="text-xl mb-4">Empower your organization with our state-of-the-art decentralized notification service. Every organization operates its own server, providing unparalleled control and security. Effortlessly create IP-Sets to group servers from different organizations, enabling you to broadcast concise notifications across your network. Receive and view all notifications on your home server with ease. While notifications are publicly viewable, login is required for sending messages and managing your IP-Sets. Experience a new era of decentralized, efficient, and secure notifications.</p>
        <div>
        <Button pill class="mr-10" size="xl" href="/received-notifications">Received Notifications</Button>
        <GradientButton pill color="purple" size="xl" href="/auth/sign-up">Sign UP</GradientButton>
        </div>
    </div>
    <div class="flex justify-center items-center p-4">
        <BrainOutline color="primary" class="text-primary-700 w-1/4 h-1/4" />
    </div>
</div>
