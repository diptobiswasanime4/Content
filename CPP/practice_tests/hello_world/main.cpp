#include <gtk/gtk.h>

// Global variables
int counter = 0;
GtkWidget *label;

// Function to handle button click
static void on_increment_button_clicked(GtkWidget *widget, gpointer data)
{
    counter++;
    // Convert counter to string
    gchar *counterStr = g_strdup_printf("%d", counter);
    // Update the label text
    gtk_label_set_text(GTK_LABEL(label), counterStr);
    g_free(counterStr);
}

// Function to handle window close
static void on_destroy(GtkWidget *widget, gpointer data)
{
    gtk_main_quit();
}

int main(int argc, char *argv[])
{
    // Initialize GTK
    gtk_init(&argc, &argv);

    // Create the main window
    GtkWidget *window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_title(GTK_WINDOW(window), "Counter App");
    gtk_window_set_default_size(GTK_WINDOW(window), 300, 200);

    // Create a vertical box to hold the label and button
    GtkWidget *box = gtk_box_new(GTK_ORIENTATION_VERTICAL, 10);
    gtk_container_add(GTK_CONTAINER(window), box);

    // Create a label to display the counter
    label = gtk_label_new("0");
    gtk_box_pack_start(GTK_BOX(box), label, TRUE, TRUE, 0);

    // Create a button to increment the counter
    GtkWidget *button = gtk_button_new_with_label("Increment");
    g_signal_connect(button, "clicked", G_CALLBACK(on_increment_button_clicked), NULL);
    gtk_box_pack_start(GTK_BOX(box), button, TRUE, TRUE, 0);

    // Connect the window destroy signal to on_destroy function
    g_signal_connect(window, "destroy", G_CALLBACK(on_destroy), NULL);

    // Show all widgets
    gtk_widget_show_all(window);

    // Start the GTK main loop
    gtk_main();

    return 0;
}
