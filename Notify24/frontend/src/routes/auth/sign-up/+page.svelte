<script lang="ts">
    import {Label, Input, Button, A, Card, Helper} from 'flowbite-svelte';
    let showInputHelperMessage = false;
    let inputHelperColor: "red" | "green" | "base" = "base";

    let password = '';
    let confirmation = '';
    const validateInput = () => {
        console.log(confirmation)
        console.log(password)
        if (password === confirmation) {
            showInputHelperMessage = false;
            inputHelperColor = "green";
            return true;
        } else if (confirmation === '') {
            showInputHelperMessage = false;
            inputHelperColor = "base";
            return false;
        }
        else {
            showInputHelperMessage = true;
            inputHelperColor = "red";
            return false;
        }
    }

</script>

<div class="flex flex-col items-center justify-center px-6 pt-8 mx-auto md:h-screen pt:mt-0 dark:bg-gray-900">
    <Card class="w-full" size="md" border={false}>
        <h1 class="flex justify-center text-2xl font-bold text-gray-900 dark:text-white">
            Create a Notifiy24 Account
        </h1>
        <form method="POST" action="/auth/sign-up?/register" class="mt-8 space-y-6">
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
                            bind:value={password}
                            on:input={validateInput}
                            type="password"
                            name="password"
                            placeholder="••••••••••"
                            required
                            class="border outline-none dark:border-gray-600 dark:bg-gray-700"
                    />
                </Label>
            </div>
            <div>
                <Label class="space-y-2 dark:text-white">
                    <span>Confirm password</span>
                    <Input
                            bind:value={confirmation}
                            on:input={validateInput}
                            type="password"
                            name="password-confirmation"
                            placeholder="••••••••••"
                            color={inputHelperColor}
                            required
                            class="border outline-none dark:border-gray-600 dark:bg-gray-700"
                    />
                </Label>
                {#if showInputHelperMessage}
                    <Helper class="mt-2" color={inputHelperColor}>
                        <span class="font-medium">Both passwords have to be the same!</span>
                    </Helper>
                {/if}
            </div>
            <Button type="submit" size="lg">Sign Up</Button>
            <div class="text-sm font-medium text-gray-500 dark:text-gray-400">
                Sign in  <A href="/auth/sign-in">here</A> if you alrleady have an account
            </div>
        </form>
    </Card>

</div>
