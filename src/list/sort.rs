use super::super::collection::Collection;
use super::slice::{slice_mut, ListMutView};
use super::{get, get_mut, iter, List, ListMut};
use core::mem;

fn swap<L: ListMut>(list: &mut L, i: usize, j: usize) {
    if i == j {
        return;
    }

    let mut t = unsafe { mem::uninitialized() };
    mem::swap(&mut t, get_mut(list, i));
    mem::swap(&mut t, get_mut(list, j));
    mem::swap(&mut t, get_mut(list, i));
    mem::forget(t);
}

#[cfg_attr(feature = "cargo-clippy", allow(nonminimal_bool))]
pub fn is_stable_sort<
    E,
    L: List<Elem = E> + Collection,
    F: Fn(&E, &E) -> bool,
    I: List<Elem = usize>,
>(
    list: &L,
    lt: &F,
    index: &I,
) -> bool {
    let size = Collection::size(list);

    if size > 0 {
        for i in 0..(size - 1) {
            if !lt(get(list, *get(index, i)), get(list, *get(index, i + 1)))
                && !(get(index, i) < get(index, i + 1))
            {
                return false;
            }
        }
    }
    true
}

pub fn bubble<E, L: ListMut<Elem = E> + Collection, F: Fn(&E, &E) -> bool>(
    list: &mut L,
    lt: &F,
) -> usize {
    let mut count = 0;
    let size = Collection::size(list);
    if size > 0 {
        for i in (1..size).rev() {
            if lt(get(list, i), get(list, i - 1)) {
                swap(list, i, i - 1);
                count += 1;
            }
        }
    }

    count
}

pub fn bubble_sort<E, L: ListMut<Elem = E> + Collection, F: Fn(&E, &E) -> bool>(
    list: &mut L,
    lt: &F,
) -> usize {
    let mut count = 0;
    let size = Collection::size(list);
    for i in 0..size - 1 {
        count += bubble(&mut slice_mut(list, i..size), lt);
    }
    count
}

pub fn bubble_sorted<E, L: ListMut<Elem = E> + Collection, F: Fn(&E, &E) -> bool>(
    list: &mut L,
    lt: &F,
) -> usize {
    let mut count = 0;
    let size = Collection::size(list);
    let mut i = size - 1;
    while (i > 0) && lt(get(list, i), get(list, i - 1)) {
        swap(list, i, i - 1);
        count += 1;
        i -= 1;
    }
    count
}

fn insertion_sort_g<E, L: ListMut<Elem = E> + Collection, F: Fn(&E, &E) -> bool>(
    list: &mut L,
    lt: &F,
    g: usize,
) -> usize {
    let mut count = 0;
    let size = Collection::size(list);

    for i in g..size {
        let mut j = i;
        while (j >= g) && lt(get(list, j), get(list, j - g)) {
            swap(list, j, j - g);
            count += 1;
            j -= g;
        }
    }

    count
}

pub fn insertion_sort<E, L: ListMut<Elem = E> + Collection, F: Fn(&E, &E) -> bool>(
    list: &mut L,
    lt: &F,
) -> usize {
    // let mut count = 0;
    // let size = Collection::size(list);
    // for i in 2..size+1 {
    //     count += bubble_sorted(slice_mut!(list, [0, i]), lt);
    // }
    // count
    insertion_sort_g(list, lt, 1)
}

pub fn shell_sort<
    E,
    L: ListMut<Elem = E> + Collection,
    F: Fn(&E, &E) -> bool,
    G: List<Elem = usize> + Collection,
>(
    list: &mut L,
    lt: &F,
    gaps: &G,
) -> usize {
    let mut count = 0;
    for g in iter(gaps) {
        // for i in 0..g {
        //     count += insertion_sort(slice_mut!(list, [i,,g]), lt);
        // }
        count += insertion_sort_g(list, lt, g);
    }
    count
}

pub fn selection_sort<E, L: ListMut<Elem = E> + Collection, F: Fn(&E, &E) -> bool>(
    list: &mut L,
    lt: &F,
) -> usize {
    let mut count = 0;
    let size = Collection::size(list);
    for i in 0..size {
        let mut min = i;
        for j in i + 1..size {
            if lt(get(list, j), get(list, min)) {
                min = j;
            }
        }

        if min != i {
            swap(list, i, min);
            count += 1;
        }
    }
    count
}

pub fn partition<E, L: ListMut<Elem = E> + Collection, F: Fn(&E, &E) -> bool>(
    list: &mut L,
    lt: &F,
) -> usize {
    let size = Collection::size(list);
    let mut i = 0;
    for j in 0..size - 1 {
        if lt(get(list, j), get(list, size - 1)) {
            swap(list, j, i);
            i += 1;
        }
    }

    swap(list, i, size - 1);
    i
}

fn quick_sort_aux<'a, 'b: 'a, E, L: ListMut<Elem = E> + Collection, F: Fn(&E, &E) -> bool>(
    list: &'b mut ListMutView<'a, L>,
    lt: &F,
) {
    let size = Collection::size(list);
    if size < 2 {
        return;
    }

    let p = partition(list, lt);
    quick_sort_aux::<E, L, F>(&mut slice_mut(list, ..p), lt);
    quick_sort_aux::<E, L, F>(&mut slice_mut(list, (p + 1)..), lt);
}

pub fn quick_sort<E, L: ListMut<Elem = E> + Collection, F: Fn(&E, &E) -> bool>(
    list: &mut L,
    lt: &F,
) {
    quick_sort_aux::<E, L, F>(&mut slice_mut(list, 0..), lt);
}
