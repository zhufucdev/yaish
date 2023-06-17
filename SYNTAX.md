> This project is under development

# Yet Another Shell Script

Yass is a domain-specific interpreted language inspired by Groovy and CSS. It is object-oriented, static and strong-typed.

In yaish, it's used for customization, stylization and syntax completion.

It's still under development, but I can give you a feeling of it.
```yass
literal('pacman -S ##') {
    where '##' selects cached {
        cache_control: watch-fs-change('/var/lib/pacman/sync/*')
        return entries cli('pacman -Ssq')
    }
}
```