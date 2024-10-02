<script>
  import "../../app.css";
  import { Navbar, NavBrand, NavLi, NavUl, Button } from 'flowbite-svelte';
  import { BrainOutline } from 'flowbite-svelte-icons';
  /** @type {import('./$types').LayoutData} */
  export let data;

  let showLoggedInStuff = data.loggedIn === true;
  let username = '';
  if (data.loggedIn === true) {
    username = data.username ? data.username : '';
  } else {
    username = '';
  }
</script>

<div class="flex flex-col min-h-screen bg-gray-50">

<Navbar class="bg-transparent">
    <NavBrand color="primary" class="w-1/6" href="/">
        <BrainOutline color="primary" class="text-primary-700" size="xl" />
        <span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">Notify24</span>
    </NavBrand>
    {#if showLoggedInStuff }
    <div class="flex items-center md:order-2">
        <p class="mx-4">{username}</p>
        <form method="POST" action="/auth/sign-out/?/logout">
            <Button color="primary" type="submit" size="sm" >Sign Out</Button>
        </form>
    </div>
    {/if}
    {#if !showLoggedInStuff }
    <div class="flex w-1/6 md:order-2">
        <Button class="mx-2" size="sm" href="/auth/sign-in">Sign In</Button>
        <Button size="sm" href="/auth/sign-up">Sign Up</Button>
    </div>
    {/if}
    <NavUl>
        <NavLi href="/">About</NavLi>
        <NavLi href="/received-notifications">Notifications</NavLi>
        {#if showLoggedInStuff }
        <NavLi href="/ip-set">Ip Sets</NavLi>
        {/if}
    </NavUl>
</Navbar>

<div class="flex column flex-col flex-grow">

<slot />
</div>
</div>
