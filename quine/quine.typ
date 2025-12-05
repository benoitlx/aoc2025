// #let prefix = "#let prefix = "
// #let suffix = "\n#let suffix = \n#raw(prefix) #raw(repr(prefix)) #raw(suffix)"
// #raw(prefix) #raw(repr(prefix)) #raw(suffix)


#let prefix = "#let prefix = "
#let suffix = "\n#let suffix = \n#raw(prefix) #raw(repr(prefix)) #raw(suffix.slice(0, 15)) #raw(repr(suffix.replace(str.from-unicode(92), str.from-unicode(92)))) #raw(suffix.slice(15))"
#raw(prefix) #raw(repr(prefix)) #raw(suffix.slice(0, 15)) #raw(repr(suffix.replace(str.from-unicode(92), str.from-unicode(92)))) #raw(suffix.slice(15))



// #let code = "#let code = " + repr("#let code = ") + "\n#raw(code)"
// #raw(code)


// #let var = "[#metadata(\"context {query(var)}\") <var>]; context {query(var)}"
// #eval(var)

// var = "print('var = ', repr(var), 'eval(var)')"
// eval(var)
