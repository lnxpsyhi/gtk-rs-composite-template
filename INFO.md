# 🎨 GTK Style Provider Priorities

GTK uses **style priorities** to determine which CSS rules override others when multiple style providers are applied.

## 📊 Priority Table

| Priority Value | Constant                             | Who Uses It           | Purpose                                                        | Should You Use It? |
|----------------|--------------------------------------|------------------------|----------------------------------------------------------------|---------------------|
| `1`            | `STYLE_PROVIDER_PRIORITY_FALLBACK`   | GTK Internals          | Fallback defaults if nothing else matches                     | ❌ No               |
| `200`          | `STYLE_PROVIDER_PRIORITY_THEME`      | GTK Theme (Adwaita)    | Default system themes                                          | ⚠️ No (theme-level) |
| `400`          | `STYLE_PROVIDER_PRIORITY_SETTINGS`   | GTK Internals          | Settings-based style tweaks                                    | ❌ No               |
| `600`          | `STYLE_PROVIDER_PRIORITY_APPLICATION`| **Your App** ✅        | For app-specific styling, overriding themes                   | ✅ Yes              |
| `800`          | `STYLE_PROVIDER_PRIORITY_USER`       | End User (`gtk.css`)   | For user customizations and overrides                         | ⚠️ Maybe (testing)  |

> 🧠 **Higher priority values override lower ones** for matching CSS selectors.

---

## ✅ Recommended Usage

Use this in Rust/GTK when loading app-level stylesheets:

```rust
gtk::style_context_add_provider_for_display(
    &gdk::Display::default().unwrap(),
    &provider,
    gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
);
