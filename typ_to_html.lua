function Link(el)
  if string.match(el.target, "%.typ$") then
    el.target = string.gsub(el.target, "%.typ$", ".html")
  end
  return el
end
