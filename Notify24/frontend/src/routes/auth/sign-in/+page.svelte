<script lang="ts">
    import {Label, Input, Button, A, Checkbox, Card, Modal} from 'flowbite-svelte';
    /** @type {import('./$types').ActionData} */
    export let form;
    let incorrect = false;

    $: if (form?.incorrect) {
        incorrect = form.incorrect;
    }
</script>

<div class="flex flex-col items-center justify-center px-6 pt-8 mx-auto md:h-screen pt:mt-0 dark:bg-gray-900">

    <Card class="w-full" size="md" border={false}>
        <h1 class="flex justify-center text-2xl font-bold text-gray-900 dark:text-white">
            Login to your Notifiy24 Account
        </h1>
        <form method="POST" action="/auth/sign-in?/login" class="mt-8 space-y-6"> <!-- on:submit|preventDefault> -->
            <div>
                <Label class="space-y-2 dark:text-white">
                    <span>Your email</span>
                    <Input
                            type="email"
                            name="email"
                            placeholder="you@notify24.com"
                            required
                            class="border outline-none dark:border-gray-600 dark:bg-gray-700"
                    />
                </Label>
            </div>
            <div>
                <Label class="space-y-2 dark:text-white">
                    <span>Your password</span>
                    <Input
                            type="password"
                            name="password"
                            placeholder="••••••••••"
                            required
                            class="border outline-none dark:border-gray-600 dark:bg-gray-700"
                    />
                </Label>
            </div>
            {#if incorrect}
                <p class="text-red-700">Sorry we couldn't log you in!</p>
            {/if}
            <Button type="submit" size="lg">Sign In</Button>
            <div class="text-sm font-medium text-gray-500 dark:text-gray-400">
                Not registered? Create your free account  <A href="/auth/sign-up">here</A>
            </div>

        </form>
    </Card>

</div>
